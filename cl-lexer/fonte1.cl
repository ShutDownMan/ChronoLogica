str = "test";

thermistor = [
    15.36 at ut1723609452,
    15.31 at ut1723609453,
    14.97 at ut1723609454,
    14.99 at ut1723609455,
];
pressure = [
    101.1 at 2024-08-14T01:24:12-03:00,
    101.2 at 2024-08-14T01:24:13-03:00,
    101.1 at 2024-08-14T01:24:14-03:00,
];

T = lerp thermistor within 1h 15m;
P = locf pressure within 1d 12h;

result = T**2 + 1 / (P + 273.15) mirroring (T, P);

