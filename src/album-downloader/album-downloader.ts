import axios from 'axios';
import fs from 'fs';
import path from 'path';

import { removeTrailingSlash } from '@utils/helpers';

import { AlbumMetainfo, TrackMetainfo } from 'src/objects';

export interface IMP3Downloader {
	downloadAlbum(downloadDirectory: string): void;
}

export class AlbumDownloader implements IMP3Downloader {
	private _album: AlbumMetainfo;
	private _downloadDirectory: string;
	private _path: string;

	public constructor(album: AlbumMetainfo, directory: string) {
		this._album = album;
		this._downloadDirectory = removeTrailingSlash(directory);
		this._path = this._makePath();
	}

	public async downloadAlbum(): Promise<void> {
		this._createDirectory();

		await this._downloadAlbumArt();

		for (const track of this._album.tracks) {
			await (await axios.get(track.url, { responseType: 'stream' }))
				.data
				.pipe(fs.createWriteStream(`${this._path}/${this._makeFileName(track)}`));
		}
	}

	private _makePath(): string {
		return path.resolve(
			`${this._downloadDirectory}/${this._album.artist}/${this._album.releaseYear} - ${this._album.title}`
		);
	}

	private _makeFileName(track: TrackMetainfo): string {
		return `${track.number}.${track.title}.mp3`;
	}

	private _createDirectory(): void {
		fs.mkdirSync(this._path, { recursive: true });
	}

	private async _downloadAlbumArt(): Promise<void> {
		await (await axios.get(this._album.albumArtURL, { responseType: 'stream' }))
			.data
			.pipe(fs.createWriteStream(`${this._path}/cover.jpg`));
	}
}
