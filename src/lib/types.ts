export interface Track {
  id: string;
  path: string;
  title: string;
  artist: string;
  album: string;
  album_artist: string;
  track_number: number;
  disc_number: number;
  duration: number;
  year: number | null;
}

export interface Album {
  id: string;
  title: string;
  artist: string;
  year: number | null;
  cover_art: string | null;
  tracks: Track[];
  total_duration: number;
}
