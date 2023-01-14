import axios from 'axios';

import { decodeDoubleQuoteHtmlEntity } from '@utils/helpers';

import { BandcampAlbumMetainfo } from 'src/bandcamp/bandcamp-objects';

export interface IBandcampParser {
	getBandcampAlbumMetainfo(albumURL: string): Promise<BandcampAlbumMetainfo>;
}

export class BandcampParser implements IBandcampParser {
	private _albumURL: string;
	private _albumMetainfoDataAttributeCodeStart = 'data-tralbum="';
	private _albumMetainfoDataAttributeCodeEnd = '}"';

	public constructor(albumURL: string) {
		this._albumURL = albumURL;
	}

	public async getBandcampAlbumMetainfo(): Promise<BandcampAlbumMetainfo> {
		const albumSourceCode = await this._getAlbumPageSourceCode();

		return JSON.parse(await this._getBandcampAlbumMetainfoJSON(albumSourceCode));
	}

	private async _getAlbumPageSourceCode(): Promise<string> {
		const response = await axios.get(this._albumURL);

		return response.data as string;
	}

	private async _getBandcampAlbumMetainfoJSON(albumPageSourceCode: string): Promise<string> {
		const albumMetainfoJSONStartId = albumPageSourceCode.indexOf(this._albumMetainfoDataAttributeCodeStart);

		if (albumMetainfoJSONStartId === -1) {
			throw new Error(`Failed to find the entry in source code: ${this._albumMetainfoDataAttributeCodeStart}`);
		}

		const temp = albumPageSourceCode.substring(albumMetainfoJSONStartId + this._albumMetainfoDataAttributeCodeStart.length);

		return decodeDoubleQuoteHtmlEntity(temp.substring(0, temp.indexOf(this._albumMetainfoDataAttributeCodeEnd) + 1));
	}
}
