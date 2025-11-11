import React, { useState } from 'react';
import { tauriService } from '../services/tauri';
import { ListMusic, Music2, Play } from 'lucide-react';

export default function MusicLibrary() {
  const [playlists, setPlaylists] = useState<string[]>([]);
  const [tracks, setTracks] = useState<string[]>([]);
  const [scTracks, setScTracks] = useState<any[]>([]);
  const [loading, setLoading] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  const loadSpotifyPlaylists = async () => {
    setLoading('playlists'); setError(null);
    try { setPlaylists(await tauriService.spotifyGetPlaylists()); }
    catch (e: any) { setError(String(e)); }
    finally { setLoading(null); }
  };

  const loadSpotifyTracks = async () => {
    setLoading('tracks'); setError(null);
    try { setTracks(await tauriService.spotifyGetTracks()); }
    catch (e: any) { setError(String(e)); }
    finally { setLoading(null); }
  };

  const loadSoundcloudTracks = async () => {
    setLoading('sc'); setError(null);
    try { setScTracks(await tauriService.soundcloudGetTracks()); }
    catch (e: any) { setError(String(e)); }
    finally { setLoading(null); }
  };

  const play = async (t: string) => {
    try { await tauriService.playTrack(t); }
    catch (e) { console.error(e); }
  };

  return (
    <div className="grid grid-cols-12 gap-4">
      <div className="col-span-12">
        <div className="flex gap-2 mb-3">
          <button className="btn" onClick={loadSpotifyPlaylists}><ListMusic size={16} className="mr-1" /> Playlists</button>
          <button className="btn" onClick={loadSpotifyTracks}><Music2 size={16} className="mr-1" /> Tracks</button>
          <button className="btn" onClick={loadSoundcloudTracks}>SoundCloud</button>
        </div>
        {loading && <p className="muted">Loading {loading}...</p>}
        {error && <p className="text-red-500">{error}</p>}
      </div>

      {playlists.length > 0 && (
        <div className="col-span-12">
          <div className="card">
            <h3 className="text-lg font-semibold mb-2">Playlists</h3>
            <ul className="space-y-2">
              {playlists.map((p, i) => (
                <li key={i} className="flex items-center justify-between rounded-xl border border-mh-border bg-mh-surface/60 px-3 py-2">
                  {p}
                </li>
              ))}
            </ul>
          </div>
        </div>
      )}

      {tracks.length > 0 && (
        <div className="col-span-12">
          <div className="card">
            <h3 className="text-lg font-semibold mb-2">Spotify Tracks</h3>
            <ul className="space-y-2">
              {tracks.map((t, i) => (
                <li key={i} className="flex items-center justify-between rounded-xl border border-mh-border bg-mh-surface/60 px-3 py-2">
                  <span>{t}</span>
                  <button className="btn btn-primary" onClick={() => play(t)}><Play size={16} /></button>
                </li>
              ))}
            </ul>
          </div>
        </div>
      )}

      {scTracks.length > 0 && (
        <div className="col-span-12">
          <div className="card">
            <h3 className="text-lg font-semibold mb-2">SoundCloud Tracks</h3>
            <pre className="whitespace-pre-wrap">
              {JSON.stringify(scTracks[0], null, 2)}
            </pre>
          </div>
        </div>
      )}
    </div>
  );
}
