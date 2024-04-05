DROP TABLE IF EXISTS meanings;
DROP TABLE IF EXISTS vietnam_readings;
DROP TABLE IF EXISTS korean_h_readings;
DROP TABLE IF EXISTS korean_r_readings;
DROP TABLE IF EXISTS pinyin_readings;
DROP TABLE IF EXISTS nanori_readings;
DROP TABLE IF EXISTS kun_readings;
DROP TABLE IF EXISTS on_readings;
DROP TABLE IF EXISTS query_codes;
DROP TABLE IF EXISTS dic_numbers;
DROP TABLE IF EXISTS character_radical;
DROP TABLE IF EXISTS codepoints;
DROP TABLE IF EXISTS variants;
DROP TABLE IF EXISTS characters;
DROP TABLE IF EXISTS radicals;
DROP TABLE IF EXISTS metadata;

CREATE TABLE metadata (
    key TEXT PRIMARY KEY NOT NULL,
    value TEXT
);

CREATE TABLE radicals (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    character TEXT 
);

CREATE TABLE characters (
    literal TEXT PRIMARY KEY NOT NULL,
    freq INTEGER,
    grade INTEGER,
    stroke_count INTEGER,
    jlpt INTEGER
);

CREATE TABLE codepoints (
    id INTEGER PRIMARY KEY NOT NULL,
    type TEXT,
    value TEXT,
    character TEXT,
    FOREIGN KEY(character) REFERENCES characters(literal)
);

CREATE TABLE variants (
    id INTEGER PRIMARY KEY NOT NULL,
    type TEXT,
    value TEXT,
    character TEXT,
    FOREIGN KEY(character) REFERENCES characters(literal)
);

CREATE TABLE character_radical (
    character TEXT,
    id_radical TEXT,
    FOREIGN KEY(character) REFERENCES characters(literal)
    FOREIGN KEY(id_radical) REFERENCES radicals(id)
);

CREATE TABLE dic_numbers (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    type TEXT,
    volume INTEGER,
    page INTEGER,
    reference TEXT,
    character TEXT,
    FOREIGN KEY(character) REFERENCES characters(literal)
);

CREATE TABLE query_codes (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    type TEXT,
    skip_misclass TEXT,
    code TEXT,
    character TEXT,
    FOREIGN KEY(character) REFERENCES characters(literal)
);

CREATE TABLE on_readings (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    reading TEXT,
    character TEXT,
    FOREIGN KEY(character) REFERENCES characters(literal)
);

CREATE INDEX idx_on_readings_reading ON on_readings (reading);

CREATE TABLE kun_readings (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    reading TEXT,
    character TEXT,
    FOREIGN KEY(character) REFERENCES characters(literal)
);

CREATE INDEX idx_kun_readings_reading ON kun_readings (reading);

CREATE TABLE nanori_readings (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    reading TEXT,
    character TEXT,
    FOREIGN KEY(character) REFERENCES characters(literal)
);

CREATE INDEX idx_nanori_readings_reading ON nanori_readings (reading);

CREATE TABLE pinyin_readings (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    reading TEXT,
    character TEXT,
    FOREIGN KEY(character) REFERENCES characters(literal)
);

CREATE TABLE korean_r_readings (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    reading TEXT,
    character TEXT,
    FOREIGN KEY(character) REFERENCES characters(literal)
);

CREATE TABLE korean_h_readings (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    reading TEXT,
    character TEXT,
    FOREIGN KEY(character) REFERENCES characters(literal)
);

CREATE TABLE vietnam_readings (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    reading TEXT,
    character TEXT,
    FOREIGN KEY(character) REFERENCES characters(literal)
);

CREATE TABLE meanings (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    meaning TEXT,
    lang TEXT,
    character TEXT,
    FOREIGN KEY(character) REFERENCES characters(literal)
);

INSERT INTO radicals (character) VALUES
("⼀"), ("⼁"), ("⼂"), ("⼃"), ("⼄"), ("⼅"), ("⼆"), ("⼇"), ("⼈"), ("⼉"), ("⼊"),
("⼋"), ("⼌"), ("⼍"),	("⼎"), ("⼏"), ("⼐"), ("⼑"), ("⼒"), ("⼓"), ("⼔"), ("⼕"),
("⼖"), ("⼗"), ("⼘"), ("⼙"), ("⼚"), ("⼛"), ("⼜"), ("⼝"),	("⼞"), ("⼟"), ("⼠"),
("⼡"), ("⼢"), ("⼣"), ("⼤"), ("⼥"), ("⼦"), ("⼧"), ("⼨"), ("⼩"), ("⼪"), ("⼫"),
("⼬"), ("⼭"),	("⼮"), ("⼯"), ("⼰"), ("⼱"), ("⼲"), ("⼳"), ("⼴"), ("⼵"), ("⼶"),
("⼷"), ("⼸"), ("⼹"), ("⼺"), ("⼻"), ("⼼"), ("⼽"),	("⼾"), ("⼿"), ("⽀"), ("⽁"),
("⽂"), ("⽃"), ("⽄"), ("⽅"), ("⽆"), ("⽇"), ("⽈"), ("⽉"), ("⽊"), ("⽋"), ("⽌"),
("⽍"),	("⽎"), ("⽏"), ("⽐"), ("⽑"), ("⽒"), ("⽓"), ("⽔"), ("⽕"), ("⽖"), ("⽗"),
("⽘"), ("⽙"), ("⽚"), ("⽛"), ("⽜"), ("⽝"),	("⽞"), ("⽟"), ("⽠"), ("⽡"), ("⽢"),
("⽣"), ("⽤"), ("⽥"), ("⽦"), ("⽧"), ("⽨"), ("⽩"), ("⽪"), ("⽫"), ("⽬"), ("⽭"),
("⽮"), ("⽯"), ("⽰"), ("⽱"), ("⽲"), ("⽳"), ("⽴"), ("⽵"), ("⽶"), ("⽷"), ("⽸"),
("⽹"), ("⽺"), ("⽻"), ("⽼"), ("⽽"),	("⽾"), ("⽿"), ("⾀"), ("⾁"), ("⾂"), ("⾃"),
("⾄"), ("⾅"), ("⾆"), ("⾇"), ("⾈"), ("⾉"), ("⾊"), ("⾋"), ("⾌"), ("⾍"),	("⾎"),
("⾏"), ("⾐"), ("⾑"), ("⾒"), ("⾓"), ("⾔"), ("⾕"), ("⾖"), ("⾗"), ("⾘"), ("⾙"),
("⾚"), ("⾛"), ("⾜"), ("⾝"),	("⾞"), ("⾟"), ("⾠"), ("⾡"), ("⾢"), ("⾣"), ("⾤"),
("⾥"), ("⾦"), ("⾧"), ("⾨"), ("⾩"), ("⾪"), ("⾫"), ("⾬"), ("⾭"),	("⾮"), ("⾯"),
("⾰"), ("⾱"), ("⾲"), ("⾳"), ("⾴"), ("⾵"), ("⾶"), ("⾷"), ("⾸"), ("⾹"), ("⾺"),
("⾻"), ("⾼"), ("⾽"),	("⾾"), ("⾿"), ("⿀"), ("⿁"), ("⿂"), ("⿃"), ("⿄"), ("⿅"),
("⿆"), ("⿇"), ("⿈"), ("⿉"), ("⿊"), ("⿋"), ("⿌"), ("⿍"),	("⿎"), ("⿏"), ("⿐"),
("⿑"), ("⿒"), ("⿓"), ("⿔"), ("⿕");
