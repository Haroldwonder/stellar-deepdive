import type { NextConfig } from "next";
import createNextIntlPlugin from "next-intl/plugin";

const withNextIntl = createNextIntlPlugin("./src/i18n/request.ts");

const nextConfig: NextConfig = {
  experimental: {
    // Tree-shake barrel-heavy packages so only the imported members are bundled
    // (e.g. a single lucide icon instead of the whole icon set).
    optimizePackageImports: [
      "lucide-react",
      "recharts",
      "date-fns",
      "framer-motion",
    ],
  },
};

export default withNextIntl(nextConfig);
