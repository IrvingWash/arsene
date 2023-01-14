import 'module-alias/register';
import { BandcampAlbumConverter } from './bandcamp/bandcamp-album-converter';

import { BandcampParser } from './bandcamp/bandcamp-parser';
import { MP3Downloader } from './mp3-downloader/mp3-downloader';

console.log('Hello, World!');

async function x(): Promise<void> {
	const a = await new BandcampParser('https://thecrinn.bandcamp.com/album/dreaming-saturn').getBandcampAlbumMetainfo();

	new MP3Downloader(new BandcampAlbumConverter().convert(a), './Music').downloadAlbum();
}

x();
