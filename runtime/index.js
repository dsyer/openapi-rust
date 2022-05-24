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
				resolve({data: data, headers: response.headers, status: response.statusCode});
			});
			response.on('error', (error) => {
				reject(error);
			});
		});
	});
}

export async function getWithHeaders(url, headers) {
	// var result = await get(url, headers);
	var result = await get(url, headers);
	if (url.includes("nginx")) {
		result["Docker-Content-Digest"] = "sha256:randomstuff";
	}
	return JSON.stringify(result);
}