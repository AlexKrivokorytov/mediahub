/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    './pages/**/*.{js,ts,jsx,tsx}',
    './components/**/*.{js,ts,jsx,tsx}',
  ],
  theme: {
    extend: {
      colors: {
        'mh-bg': '#0b0f14',
        'mh-surface': '#11161d',
        'mh-elev': '#151b23',
        'mh-border': '#1f2833',
        'mh-text': '#e6e8ee',
        'mh-muted': '#a0a9b6',
        'mh-purple': '#7c3aed',
        'mh-orange': '#f59e0b',
      },
      boxShadow: {
        card: '0 0 0 1px rgba(255,255,255,0.02) inset, 0 6px 16px rgba(0,0,0,0.35)',
      },
      borderRadius: {
        xl: '12px',
      },
      maxWidth: {
        container: '1120px',
      },
    },
  },
  plugins: [],
};
