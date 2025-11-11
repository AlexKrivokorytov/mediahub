import React, { useState } from 'react';
import { tauriService } from '../services/tauri';
import { Send } from 'lucide-react';

export default function DiscordPanel() {
  const [channelId, setChannelId] = useState('');
  const [content, setContent] = useState('');
  const [status, setStatus] = useState('');

  const send = async () => {
    try {
      const res = await tauriService.discordSendMessage({ channel_id: channelId, content });
      setStatus(res);
    } catch (e: any) {
      setStatus(String(e));
    }
  };

  return (
    <section className="card">
      <div className="flex gap-2 mb-2 items-stretch w-full">
        <input className="input" placeholder="Channel ID" value={channelId} onChange={(e) => setChannelId(e.target.value)} />
        <input className="input" placeholder="Message" value={content} onChange={(e) => setContent(e.target.value)} />
        <button className="btn btn-primary" onClick={send}><Send size={16} className="mr-1" />Send</button>
      </div>
      {status && <p className="muted">{status}</p>}
    </section>
  );
}
