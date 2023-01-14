export interface BandcampAlbumMetainfo {
	current: {
		title: string;
		type: string;
		release_date: string;
	},
	hasAudio: boolean;
	artist: string;
	trackinfo: BandcampTrackInfo[];
}

export interface BandcampTrackInfo {
	file: {
		"mp3-128": string;
	},
	title: string;
	track_num: number;
	has_lyrics: boolean;
}
