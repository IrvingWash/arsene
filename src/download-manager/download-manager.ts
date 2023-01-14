import { AlbumDownloader } from '../album-downloader/album-downloader';
import { CommonParser } from '../parsers/common/common-parser';
import { parserFactory } from '../parsers/parser-factory';

export interface IDownloadManager {
	download(): Promise<void>;
}

export class DownloadManager implements IDownloadManager {
	private _url: string;
	private _downloadDirectory: string;

	private _parser: CommonParser;

	public constructor(url: string, downloadDirectory: string) {
		this._url = url;

		if (!url.includes('bandcamp')) {
			throw new Error('Currently only downloads from bandcamp are supported');
		}

		this._downloadDirectory = downloadDirectory;

		this._parser = parserFactory(this._url);
	}

	public async download(): Promise<void> {
		const data = await this._parser.getAlbumMetainfo();

		await new AlbumDownloader(data, this._downloadDirectory).downloadAlbum();
	}
}
