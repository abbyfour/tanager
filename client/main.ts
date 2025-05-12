import { Subsonic } from "./lib/Subsonic.ts";

async function main() {
  const subsonic = new Subsonic();
  const np = await subsonic.getNowPlaying();
  console.log(np);
}

main();
