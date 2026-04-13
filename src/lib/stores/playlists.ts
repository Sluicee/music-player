import { writable, get } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';
import type { Track } from '../types';
import { albums } from './library';

export interface Playlist {
  id: string;
  name: string;
  tracks: Track[];
}

const PLAYLISTS_KEY = 'mp_playlists';

function loadPlaylists(): Playlist[] {
  try {
    const saved = JSON.parse(localStorage.getItem(PLAYLISTS_KEY) ?? 'null');
    if (Array.isArray(saved) && saved.length > 0) return saved;
  } catch {}
  return [{ id: 'favourites', name: 'Favourites', tracks: [] }];
}

export const playlists = writable<Playlist[]>(loadPlaylists());

playlists.subscribe((value) => {
  localStorage.setItem(PLAYLISTS_KEY, JSON.stringify(value));
});

export function addToPlaylist(playlistId: string, track: Track) {
  playlists.update((pls) =>
    pls.map((pl) => {
      if (pl.id !== playlistId) return pl;
      if (pl.tracks.some((t) => t.id === track.id)) return pl;
      return { ...pl, tracks: [...pl.tracks, track] };
    })
  );
}

export function createPlaylist(name: string): string {
  const id = `pl_${Date.now()}`;
  playlists.update((pls) => [...pls, { id, name: name.trim(), tracks: [] }]);
  return id;
}

export function isTrackInPlaylist(playlistId: string, trackId: string): boolean {
  return get(playlists)
    .find((pl) => pl.id === playlistId)
    ?.tracks.some((t) => t.id === trackId) ?? false;
}

export function reorderPlaylistTrack(playlistId: string, fromIdx: number, toIdx: number) {
  playlists.update((pls) =>
    pls.map((pl) => {
      if (pl.id !== playlistId) return pl;
      const tracks = [...pl.tracks];
      const [moved] = tracks.splice(fromIdx, 1);
      tracks.splice(toIdx, 0, moved);
      return { ...pl, tracks };
    })
  );
}

function fileExt(path: string): string {
  return path.split('.').pop()?.toLowerCase() ?? '';
}

function buildM3U(playlist: Playlist): string {
  const lines = ['#EXTM3U'];
  for (const track of playlist.tracks) {
    lines.push(`#EXTINF:${Math.round(track.duration)},${track.artist} - ${track.title}`);
    lines.push(track.path);
  }
  return lines.join('\r\n');
}

function buildCSV(playlist: Playlist): string {
  const escape = (s: string) => `"${s.replace(/"/g, '""')}"`;
  const rows = ['path,title,artist,album,duration'];
  for (const track of playlist.tracks) {
    rows.push([
      escape(track.path),
      escape(track.title),
      escape(track.artist),
      escape(track.album),
      String(Math.round(track.duration)),
    ].join(','));
  }
  return rows.join('\r\n');
}

function buildTXT(playlist: Playlist): string {
  return playlist.tracks.map((t) => t.path).join('\r\n');
}

function parsePathsFromM3U(content: string): string[] {
  return content.split(/\r?\n/).map((l) => l.trim()).filter((l) => l && !l.startsWith('#'));
}

// ── CSV parsing ───────────────────────────────────────────────────────────────

/** Split one CSV line into cells, handling quoted fields. */
function splitCSVLine(line: string): string[] {
  const cells: string[] = [];
  let cur = '';
  let inQ = false;
  for (let i = 0; i < line.length; i++) {
    const ch = line[i];
    if (inQ) {
      if (ch === '"' && line[i + 1] === '"') { cur += '"'; i++; }
      else if (ch === '"') { inQ = false; }
      else { cur += ch; }
    } else {
      if (ch === '"') { inQ = true; }
      else if (ch === ',') { cells.push(cur); cur = ''; }
      else { cur += ch; }
    }
  }
  cells.push(cur);
  return cells;
}

/** Column-name aliases for common external playlist formats. */
const COL_TITLE  = ['track name', 'title', 'name', 'song', 'song name', 'track title'];
const COL_ARTIST = ['artist name(s)', 'artist name', 'artist', 'artists', 'performer'];
const COL_ALBUM  = ['album name', 'album', 'release'];

interface CSVMeta { title: string; artist: string; album: string }

/** Parse CSV — returns path list (internal) or metadata list (external). */
function parseCSV(content: string):
  | { kind: 'paths'; paths: string[] }
  | { kind: 'meta';  items: CSVMeta[] }
{
  // Strip BOM
  const raw = content.replace(/^\uFEFF/, '');
  const lines = raw.split(/\r?\n/).filter((l) => l.trim());
  if (lines.length < 2) return { kind: 'paths', paths: [] };

  const header = splitCSVLine(lines[0]).map((h) => h.trim().toLowerCase());

  const pathCol = header.indexOf('path');
  if (pathCol !== -1) {
    // Internal format
    const paths = lines.slice(1).map((l) => {
      const c = splitCSVLine(l);
      return (c[pathCol] ?? '').trim();
    }).filter(Boolean);
    return { kind: 'paths', paths };
  }

  // External format — find title/artist/album columns
  const titleCol  = COL_TITLE.map((n) => header.indexOf(n)).find((i) => i !== -1) ?? -1;
  const artistCol = COL_ARTIST.map((n) => header.indexOf(n)).find((i) => i !== -1) ?? -1;
  const albumCol  = COL_ALBUM.map((n) => header.indexOf(n)).find((i) => i !== -1) ?? -1;

  if (titleCol === -1) return { kind: 'paths', paths: [] };

  const items: CSVMeta[] = lines.slice(1).map((l) => {
    const c = splitCSVLine(l);
    return {
      title:  (c[titleCol]  ?? '').trim(),
      artist: (c[artistCol] ?? '').trim(),
      album:  (c[albumCol]  ?? '').trim(),
    };
  }).filter((r) => r.title);

  return { kind: 'meta', items };
}

// ── Metadata matching ─────────────────────────────────────────────────────────

function normStr(s: string): string {
  return s
    .toLowerCase()
    .replace(/[\u2010-\u2015\u2212]/g, '-')   // fancy dashes → hyphen
    .replace(/\s*\([^)]*\)/g, '')             // remove (...)
    .replace(/\s*\[[^\]]*\]/g, '')            // remove [...]
    .replace(/[.,!?;:'"「」『』【】（）]/g, '')
    .replace(/\s+/g, ' ')
    .trim();
}

/** Check if a CSV artist field (may be semicolon-separated) matches a local artist. */
function artistMatches(csvArtist: string, localArtist: string): boolean {
  if (!csvArtist) return true;
  const normLocal = normStr(localArtist);
  return csvArtist.split(/[;]/).map((a) => normStr(a.trim())).some(
    (a) => a === normLocal || normLocal.includes(a) || a.includes(normLocal)
  );
}

function findByMeta(allTracks: Track[], { title, artist, album }: CSVMeta): Track | undefined {
  const nt = normStr(title);
  const na = normStr(album);

  // 1. title + artist + album
  const m1 = allTracks.find(
    (t) => normStr(t.title) === nt && artistMatches(artist, t.artist) && normStr(t.album) === na
  );
  if (m1) return m1;

  // 2. title + artist
  const m2 = allTracks.find(
    (t) => normStr(t.title) === nt && artistMatches(artist, t.artist)
  );
  if (m2) return m2;

  // 3. title + album (for compilations / various artists)
  if (na) {
    const m3 = allTracks.find((t) => normStr(t.title) === nt && normStr(t.album) === na);
    if (m3) return m3;
  }

  return undefined;
}

function parsePathsFromTXT(content: string): string[] {
  return content.split(/\r?\n/).map((l) => l.trim()).filter(Boolean);
}

// ── Public API ────────────────────────────────────────────────────────────────

export async function exportPlaylist(playlist: Playlist): Promise<boolean> {
  const safeName = playlist.name.replace(/[<>:"/\\|?*]/g, '').trim() || 'playlist';
  const path = await invoke<string | null>('pick_save_file', { defaultName: `${safeName}.m3u` });
  if (!path) return false;

  const ext = fileExt(path);
  let content: string;
  if (ext === 'csv') content = buildCSV(playlist);
  else if (ext === 'txt') content = buildTXT(playlist);
  else content = buildM3U(playlist);

  await invoke('write_file', { path, content });
  return true;
}

export async function importIntoPlaylist(playlistId: string): Promise<number> {
  const path = await invoke<string | null>('pick_open_file');
  if (!path) return 0;

  const content = await invoke<string>('read_file', { path });
  const ext = fileExt(path);
  const allTracks = get(albums).flatMap((a) => a.tracks);

  // Collect candidates
  const candidates: Track[] = [];

  if (ext === 'csv') {
    const parsed = parseCSV(content);
    if (parsed.kind === 'paths') {
      for (const fp of parsed.paths) {
        const norm = fp.replace(/\\/g, '/');
        const t = allTracks.find((t) => t.path === fp || t.path.replace(/\\/g, '/') === norm);
        if (t) candidates.push(t);
      }
    } else {
      for (const meta of parsed.items) {
        const t = findByMeta(allTracks, meta);
        if (t) candidates.push(t);
      }
    }
  } else if (ext === 'txt') {
    for (const fp of parsePathsFromTXT(content)) {
      const norm = fp.replace(/\\/g, '/');
      const t = allTracks.find((t) => t.path === fp || t.path.replace(/\\/g, '/') === norm);
      if (t) candidates.push(t);
    }
  } else {
    for (const fp of parsePathsFromM3U(content)) {
      const norm = fp.replace(/\\/g, '/');
      const t = allTracks.find((t) => t.path === fp || t.path.replace(/\\/g, '/') === norm);
      if (t) candidates.push(t);
    }
  }

  let added = 0;
  playlists.update((pls) =>
    pls.map((pl) => {
      if (pl.id !== playlistId) return pl;
      let tracks = [...pl.tracks];
      for (const track of candidates) {
        if (!tracks.some((t) => t.id === track.id)) {
          tracks = [...tracks, track];
          added++;
        }
      }
      return { ...pl, tracks };
    })
  );

  return added;
}
