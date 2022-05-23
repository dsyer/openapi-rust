import http from 'http';

function get(url, headers) {
	return new Promise((resolve, reject) => {
		http.get(url, { "headers": headers }, response => {
			response.setEncoding('utf8');
			let data = '';
			response.on('data', (chunk) => {
				data += chunk.toString();
			});
			response.on('end', () => {
				resolve(data);
			});
			response.on('error', (error) => {
				reject(error);
			});
		});
	});
}

export function getWithHeaders(url, headers) {
	// var result = await get(url, headers);
	var result = {};
	if (url.includes("nginx")) {
		result["Docker-Content-Digest"] = "sha256:randomstuff";
	}
	return JSON.stringify({ "headers": result });
}