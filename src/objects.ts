export interface AlbumMetainfo {
	title: string;
	artist: string;
	releaseYear: number;
	tracks: TrackMetainfo[];
}

export interface TrackMetainfo {
	title: string;
	number: string;
	url: string;
}
