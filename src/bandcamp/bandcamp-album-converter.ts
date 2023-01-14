import { AlbumMetainfo, TrackMetainfo } from 'src/objects';

import { BandcampAlbumMetainfo } from './bandcamp-objects';

export class BandcampAlbumConverter {
	public convert(bandcampAlbum: BandcampAlbumMetainfo): AlbumMetainfo {
		if (!bandcampAlbum.hasAudio) {
			throw new Error('Album does not have audio');
		}

		const {
			artist,
			current,
			trackinfo,
		} = bandcampAlbum;

		const tracks: TrackMetainfo[] = [];

		for (const track of trackinfo) {
			const { track_num, title, file } = track;

			tracks.push({
				number: track_num > 9 ? String(track_num) : `0${track_num}`,
				title,
				url: file['mp3-128'],
			});
		}

		return {
			artist: artist,
			title: current.title,
			releaseYear: new Date(current.release_date).getFullYear(),
			tracks,
		};
	}
}
