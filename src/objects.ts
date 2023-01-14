export interface AlbumMetainfo {
	title: string;
	artist: string;
	releaseYear: number;
	tracks: TrackMetainfo[];
	albumArtURL: string;
}

export interface TrackMetainfo {
	title: string;
	number: string;
	url: string;
}
