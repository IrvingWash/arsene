import 'module-alias/register';

import { DownloadManager } from './download-manager/download-manager';

async function start(): Promise<void> {
	const dm = new DownloadManager(
		'https://thecrinn.bandcamp.com/album/dreaming-saturn',
		'./Music'
	);

	await dm.download();
}

start();
