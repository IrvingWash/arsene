import { AlbumMetainfo } from 'src/objects';

export abstract class CommonParser {
	protected _albumURL: string;

	public constructor(albumURL: string) {
		this._albumURL = albumURL;
	}

	public abstract getAlbumMetainfo(): Promise<AlbumMetainfo>;
}
