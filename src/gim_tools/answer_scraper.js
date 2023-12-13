(async () => {
	function extractKey(object, targetKey) {
		if (typeof object !== "object" || object === null) return undefined;
		if (targetKey in object) return object[targetKey];
		for (const key in object) {
			const result = extractKey(object[key], targetKey);
			if (result !== undefined) return result;
		}
		return undefined;
	}

	function waitForElement(selector) {
		return new Promise((resolve) => {
			const interval = setInterval(() => {
				if (document.querySelector(selector)) {
					clearInterval(interval);
					resolve();
				}
			}, 50);
		});
	}

	waitForElement(".flex.vx").then(async () => {
		let answerDiv = document.querySelector(".flex.vx");
		let key = Object.keys(answerDiv)[1];
		let game = extractKey(answerDiv[key], "game");
		let json = await fetch(`https://www.gimkit.com/api/games/fetch/${game}`).then((response) => response.json());

		let kit = document.createElement("div");
		kit.setAttribute("id", "kit");
		kit.setAttribute("style", "display: none;");
		kit.setAttribute("data", JSON.stringify(json));

		document.body.appendChild(kit);
	});
})();
