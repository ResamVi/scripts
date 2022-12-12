// ==UserScript==
// @name         FreshRSS to YT-Playlist
// @namespace    http://tampermonkey.net/
// @version      0.1
// @description  Because clicking all links is annoying.
// @author       ResamVi (Julien Midedji)
// @match        https://feed.resamvi.io
// @grant        none
// ==/UserScript==

(function() {
    'use strict';
	//
	// Create button
	const btn = document.createElement("button");
	btn.innerHTML = "Hello Button";

	// When button is clicked
	btn.onclick = () => { 
		console.log("Hello");

		// Get all items's links in list
		const elements = document.body.querySelectorAll('li.item.link > a');

		// Get the video's ID
		let videos = [];
		elements.forEach((element, index) => {
			if(index % 2 == 0) return; // idk why they show twice
			videos.push(element.href.split("=")[1]);
		});

		// And create a playlist
		let url = "http://www.youtube.com/watch_videos?video_ids=" + videos.join(",");

		// That we open in a new tab
		window.open(url, '_blank').focus();
	};

	// Create container for button
	const div = document.createElement("div");
	div.classList.add("item");
	div.appendChild(btn);

	// Put Button on header
	const header = document.getElementsByClassName("header")[0];
	header.appendChild(div);
})();
