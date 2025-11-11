/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  // Enable static export so Tauri can use built assets from `out/`
  output: 'export',
};

export default nextConfig;

