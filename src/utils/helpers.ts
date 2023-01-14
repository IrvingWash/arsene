export function decodeDoubleQuoteHtmlEntity(value: string): string {
	return value.replace(/&quot;/g, '"');
}

export function removeTrailingSlash(value: string): string {
	const lastCharacterIndex = value.length - 1;

	return value.charAt(lastCharacterIndex) !== '/'
		? value
		: value.slice(0, lastCharacterIndex);
}
