// /** @type {import('next').NextConfig} */
//
// const nextConfig = {
//   reactStrictMode: true,
//   // Note: This feature is required to use NextJS Image in SSG mode.
//   // See https://nextjs.org/docs/messages/export-image-api for different workarounds.
//   images: {
//     unoptimized: true,
//   },
// }
//
// module.exports = nextConfig
/** @type {import('next').NextConfig} */
const nextConfig = {
  output: "export",
  env: {
    ns: "TuringApp",
    db: "TuringDB",
    sc: "TuringScope",
    host: "localhost",
    port: 8080,
  },
};

module.exports = nextConfig;
