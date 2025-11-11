import React, { useEffect, useState } from 'react';
import { tauriService } from '../services/tauri';
import { Play, Pause, Volume2 } from 'lucide-react';

export default function NowPlaying() {
  const [status, setStatus] = useState('Paused');
  const [volume, setVolume] = useState(0.5);
  const [trackUrl, setTrackUrl] = useState('');

  useEffect(() => {
    // Load last URL/volume from localStorage
    const v = localStorage.getItem('player.volume');
    const u = localStorage.getItem('player.trackUrl');
    if (v) setVolume(parseFloat(v));
    if (u) setTrackUrl(u);
  }, []);

  const play = async () => {
    if (!trackUrl) return;
    await tauriService.playTrack(trackUrl);
    localStorage.setItem('player.trackUrl', trackUrl);
    setStatus('Playing');
  };

  const pause = async () => {
    await tauriService.pauseTrack();
    setStatus('Paused');
  };

  const changeVolume = async (v: number) => {
    setVolume(v);
    localStorage.setItem('player.volume', String(v));
    await tauriService.setVolume(v);
  };

  return (
    <section className="card">
      <h3 className="text-lg font-semibold mb-1">Now Playing</h3>
      <div className="muted mb-2">Status: {status}</div>
      <div className="flex items-stretch gap-2">
        <input
          className="input"
          placeholder="Enter track URL (Spotify/SoundCloud)"
          value={trackUrl}
          onChange={(e) => setTrackUrl(e.target.value)}
        />
        <button className="btn btn-primary" onClick={play}><Play size={16} className="mr-1" />Play</button>
        <button className="btn" onClick={pause}><Pause size={16} className="mr-1" />Pause</button>
      </div>
      <div className="mt-3">
        <div className="flex items-center justify-between">
          <span className="muted inline-flex items-center gap-1.5">
            <Volume2 size={16} /> Volume
          </span>
          <span className="muted">{(volume * 100) | 0}%</span>
        </div>
        <input className="w-full accent-mh-purple" type="range" min={0} max={1} step={0.01} value={volume} onChange={(e) => changeVolume(parseFloat(e.target.value))} />
      </div>
    </section>
  );
}
