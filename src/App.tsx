import React, { useEffect, useState } from "react";
import { MusicLibrary } from "./components/MusicLibrary";
import { AnimeTracker } from "./components/AnimeTracker";
import { DiscordPanel } from "./components/DiscordPanel";
import { NowPlaying } from "./components/NowPlaying";
import { tauriService } from "./services/tauri";

export default function App() {
  const [status, setStatus] = useState<string>("initializing...");

  useEffect(() => {
    const checks = [
      {
        name: "Spotify",
        task: () => tauriService.getSpotifyPlaylists()
      },
      {
        name: "Shikimori",
        task: () => tauriService.getAnimeList()
      },
      {
        name: "SoundCloud",
        task: () => tauriService.getSoundcloudTracks()
      },
      {
        name: "Player",
        task: () => tauriService.pauseTrack()
      }
    ];

    (async () => {
      const results = await Promise.allSettled(checks.map((c) => c.task()));
      const summary = results.map((res, idx) => {
        const label = checks[idx].name;
        if (res.status === "fulfilled") {
          if (Array.isArray(res.value)) {
            return `${label}: ${res.value.length} items`;
          }
          return `${label}: ok`;
        }
        return `${label}: ${String(res.reason)}`;
      });
      setStatus(summary.join(" | "));
    })();
  }, []);

  return (
    <div style={{ padding: 16, fontFamily: "system-ui, sans-serif" }}>
      <h1>Media Hub</h1>
      <div style={{ marginBottom: 12 }}>Backend status: {status}</div>
      <NowPlaying />
      <div style={{ display: "grid", gridTemplateColumns: "1fr 1fr", gap: 16 }}>
        <MusicLibrary />
        <AnimeTracker />
      </div>
      <div style={{ marginTop: 16 }}>
        <DiscordPanel />
      </div>
    </div>
  );
}


