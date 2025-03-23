create only bus_route:jo set name = "Soubu";
create only bus_route:jk set name = "Keihin Tohoku";
create only bus_stop:kaijin set name = "kaijin";
create only bus_stop:funa set name = "Funabashi";
create only bus_stop:ichi set name = "Ichikawa";
create only bus_stop:shimbashi set name = "Shimbashi";
create only bus_stop:tokyo set name = "Tokyo";
relate bus_route:jo->contain->bus_stop:kaijin set index = 1;
relate bus_route:jo->contain->bus_stop:funa set index = 2;
relate bus_route:jo->contain->bus_stop:ichi set index = 3;
relate bus_route:jo->contain->bus_stop:shimbashi set index = 5;
relate bus_route:jo->contain->bus_stop:tokyo set index = 4;
relate bus_route:jk->contain->bus_stop:shimbashi set index = 1;
relate bus_route:jk->contain->bus_stop:tokyo set index = 2;
update bus_route set updated_at='2025-03-22T15:30:14.182205620Z', created_at='2025-03-22T15:30:14.182205620Z' where updated_at=none;
update bus_stop set updated_at='2025-03-22T15:30:14.182205620Z', created_at='2025-03-22T15:30:14.182205620Z' where updated_at=none;

SELECT meta::id(id) AS id,
    name,
    created_at,
    updated_at,
    (SELECT out.id AS id, out.name AS name, index FROM ->contain) AS stops
FROM bus_route;
