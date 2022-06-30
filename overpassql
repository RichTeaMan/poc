/*
This is overpass ql
*/
[out:json][timeout:250];
(
	//node["highway"="motorway"];
	way["highway"="motorway"]({{bbox}});
	//relation["highway"="motorway"];
);
out body;
>;
out skel qt;

