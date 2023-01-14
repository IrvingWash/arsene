import { BandcampParser } from './bandcamp/bandcamp-parser';
import { CommonParser } from './common/common-parser';

export function parserFactory(url: string): CommonParser {
	if (url.includes('bandcamp')) {
		return new BandcampParser(url);
	} else {
		throw new Error(`A parser for ${url} is not implemented yet`);
	}
}
