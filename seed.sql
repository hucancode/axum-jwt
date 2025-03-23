-- create route and its stops
let $yt = CREATE ONLY bus_route SET name = 'Yamanote', created_at = time::now(), updated_at = time::now();
let $shinjuku = CREATE ONLY bus_stop SET name = 'Shinjuku', created_at = time::now(), updated_at = time::now();
let $shibuya = CREATE ONLY bus_stop SET name = 'Shibuya', created_at = time::now(), updated_at = time::now();
let $harajuku = CREATE ONLY bus_stop SET name = 'Harajuku', created_at = time::now(), updated_at = time::now();
let $otemachi = CREATE ONLY bus_stop SET name = 'Otemachi', created_at = time::now(), updated_at = time::now();
let $tokyo = CREATE ONLY bus_stop SET name = 'Tokyo', created_at = time::now(), updated_at = time::now();
let $shinagawa = CREATE ONLY bus_stop SET name = 'Shinagawa', created_at = time::now(), updated_at = time::now();
RELATE $yt->contain->$shinjuku set index = 1;
RELATE $yt->contain->$shibuya set index = 2;
RELATE $yt->contain->$harajuku set index = 3;
RELATE $yt->contain->$otemachi set index = 4;
RELATE $yt->contain->$tokyo set index = 5;
RELATE $yt->contain->$shinagawa set index = 6;

let $jo = CREATE ONLY bus_route SET name = 'Soubu', created_at = time::now(), updated_at = time::now();
let $chiba = CREATE ONLY bus_stop SET name = 'Chiba', created_at = time::now(), updated_at = time::now();
let $funabashi = CREATE ONLY bus_stop SET name = 'Funabashi', created_at = time::now(), updated_at = time::now();
RELATE $jo->contain->$chiba set index = 1;
RELATE $jo->contain->$funabashi set index = 2;
RELATE $jo->contain->$tokyo set index = 3;
RELATE $jo->contain->$shinagawa set index = 4;

-- select all route and its stops
SELECT meta::id(id) AS id,
    name,
    created_at,
    updated_at,
    (SELECT out.id AS id, out.name AS name, index FROM ->contain) AS stops
FROM bus_route;
