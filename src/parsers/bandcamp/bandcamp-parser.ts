import axios from 'axios';

import { decodeDoubleQuoteHtmlEntity } from '@utils/helpers';

import { AlbumMetainfo } from 'src/objects';

import { CommonParser } from '../common/common-parser';
import { BandcampAlbumConverter } from './bandcamp-album-converter';
import { BandcampAlbumMetainfo } from './bandcamp-objects';

export class BandcampParser extends CommonParser {
	public constructor(albumURL: string) {
		super(albumURL);
	}

	public async getAlbumMetainfo(): Promise<AlbumMetainfo> {
		const albumSourceCode = await this._getAlbumPageSourceCode();

		const bandcampAlbum = this._getBandcampAlbumMetainfo(albumSourceCode);

		const albumArtURL = this._getAlbumArtURL(albumSourceCode);

		return new BandcampAlbumConverter().convert(bandcampAlbum, albumArtURL);
	}

	private async _getAlbumPageSourceCode(): Promise<string> {
		const response = await axios.get(this._albumURL);

		return response.data as string;
	}

	private _getBandcampAlbumMetainfo(albumPageSourceCode: string): BandcampAlbumMetainfo {
		const albumMetainfoDataAttributeCodeStart = 'data-tralbum="';
		const albumMetainfoDataAttributeCodeEnd = '}"';

		const albumMetainfoJSONStartId = albumPageSourceCode.indexOf(albumMetainfoDataAttributeCodeStart);

		if (albumMetainfoJSONStartId === -1) {
			throw new Error(`Failed to find the entry in source code: ${albumMetainfoDataAttributeCodeStart}`);
		}

		const temp = albumPageSourceCode.substring(albumMetainfoJSONStartId + albumMetainfoDataAttributeCodeStart.length);

		return JSON.parse(decodeDoubleQuoteHtmlEntity(temp.substring(0, temp.indexOf(albumMetainfoDataAttributeCodeEnd) + 1)));
	}

	private _getAlbumArtURL(albumPageSourceCode: string): string {
		const urlCodeStart = 'src="';
		const urlCodeEnd = 'jpg"';

		const albumArtCodeChunk = albumPageSourceCode.substring(albumPageSourceCode.indexOf('<div id="tralbumArt">'));

		const urlStartId = albumArtCodeChunk.indexOf(urlCodeStart);
		const urlEndId = albumArtCodeChunk.indexOf(urlCodeEnd);

		return albumArtCodeChunk.substring(urlStartId + urlCodeStart.length, urlEndId + urlCodeEnd.length + 1);
	}
}
