-- Create Dewey Decimal Classification reference table
CREATE TABLE dewey_classifications (
    id CHAR(36) PRIMARY KEY,
    code VARCHAR(20) NOT NULL UNIQUE,
    level INT NOT NULL COMMENT '1=main class, 2=division, 3=section, 4=subsection',
    parent_code VARCHAR(20) COMMENT 'Parent classification code for hierarchy',
    name VARCHAR(200) NOT NULL,
    description TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    INDEX idx_code (code),
    INDEX idx_parent (parent_code),
    INDEX idx_level (level),
    FULLTEXT INDEX ft_search (name, description)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Insert Main Classes (000-900)
INSERT INTO dewey_classifications (id, code, level, parent_code, name, description) VALUES
(UUID(), '000', 1, NULL, 'Computer Science, Information & General Works', 'Encyclopedias, libraries, museums, journalism, computer science'),
(UUID(), '100', 1, NULL, 'Philosophy & Psychology', 'Ethics, paranormal phenomena, psychology, logic'),
(UUID(), '200', 1, NULL, 'Religion', 'Theology, Bible, Christian denominations, mythology'),
(UUID(), '300', 1, NULL, 'Social Sciences', 'Sociology, anthropology, statistics, politics, economics, law, education, customs'),
(UUID(), '400', 1, NULL, 'Language', 'Linguistics, languages, dictionaries'),
(UUID(), '500', 1, NULL, 'Science', 'Mathematics, astronomy, physics, chemistry, earth sciences, biology'),
(UUID(), '600', 1, NULL, 'Technology', 'Medicine, engineering, agriculture, home economics, business'),
(UUID(), '700', 1, NULL, 'Arts & Recreation', 'Architecture, sculpture, painting, music, sports, games'),
(UUID(), '800', 1, NULL, 'Literature', 'Rhetoric, collections, American literature, English literature, German literature'),
(UUID(), '900', 1, NULL, 'History & Geography', 'World history, geography, biography');

-- Insert Popular Divisions (most commonly used in personal libraries)
INSERT INTO dewey_classifications (id, code, level, parent_code, name, description) VALUES
-- Computer Science (000s)
(UUID(), '004', 2, '000', 'Data Processing & Computer Science', 'Computer hardware, networks, programming'),
(UUID(), '005', 2, '000', 'Computer Programming', 'Software, programming languages, algorithms'),
(UUID(), '006', 2, '000', 'Special Computer Methods', 'Graphics, multimedia, artificial intelligence'),

-- Philosophy & Psychology (100s)
(UUID(), '150', 2, '100', 'Psychology', 'Consciousness, emotions, personality, developmental psychology'),
(UUID(), '170', 2, '100', 'Ethics', 'Moral philosophy, right and wrong'),

-- Religion (200s)
(UUID(), '220', 2, '200', 'Bible', 'Old Testament, New Testament, biblical studies'),
(UUID(), '230', 2, '200', 'Christianity', 'Christian theology and practice'),
(UUID(), '290', 2, '200', 'Other Religions', 'Comparative religion, mythology'),

-- Social Sciences (300s)
(UUID(), '320', 2, '300', 'Political Science', 'Politics, government, international relations'),
(UUID(), '330', 2, '300', 'Economics', 'Economic theory, labor, finance, trade'),
(UUID(), '340', 2, '300', 'Law', 'Legal systems, international law'),
(UUID(), '370', 2, '300', 'Education', 'Teaching, schools, higher education'),
(UUID(), '390', 2, '300', 'Customs & Folklore', 'Etiquette, folklore, traditions'),

-- Language (400s)
(UUID(), '420', 2, '400', 'English Language', 'English grammar, etymology'),
(UUID(), '430', 2, '400', 'German Language', 'German grammar, dictionaries'),
(UUID(), '440', 2, '400', 'French Language', 'French grammar, dictionaries'),
(UUID(), '450', 2, '400', 'Italian Language', 'Italian grammar, dictionaries'),
(UUID(), '460', 2, '400', 'Spanish Language', 'Spanish grammar, dictionaries'),

-- Science (500s)
(UUID(), '510', 2, '500', 'Mathematics', 'Arithmetic, algebra, geometry, calculus'),
(UUID(), '520', 2, '500', 'Astronomy', 'Stars, planets, cosmology'),
(UUID(), '530', 2, '500', 'Physics', 'Mechanics, energy, quantum physics'),
(UUID(), '540', 2, '500', 'Chemistry', 'Organic chemistry, inorganic chemistry'),
(UUID(), '550', 2, '500', 'Earth Sciences', 'Geology, meteorology, paleontology'),
(UUID(), '560', 2, '500', 'Paleontology', 'Fossils, prehistoric life'),
(UUID(), '570', 2, '500', 'Life Sciences & Biology', 'Botany, zoology, microbiology'),
(UUID(), '580', 2, '500', 'Plants (Botany)', 'Plant biology, flowers, trees'),
(UUID(), '590', 2, '500', 'Animals (Zoology)', 'Mammals, birds, reptiles, insects'),

-- Technology (600s)
(UUID(), '610', 2, '600', 'Medicine & Health', 'Anatomy, diseases, pharmacology, nursing'),
(UUID(), '620', 2, '600', 'Engineering', 'Mechanical, electrical, civil engineering'),
(UUID(), '630', 2, '600', 'Agriculture', 'Farming, crops, livestock'),
(UUID(), '640', 2, '600', 'Home & Family Management', 'Housekeeping, parenting, family'),
(UUID(), '650', 2, '600', 'Management & Business', 'Business administration, marketing, accounting'),
(UUID(), '660', 2, '600', 'Chemical Engineering', 'Industrial chemistry, biotechnology'),

-- Arts & Recreation (700s)
(UUID(), '710', 2, '700', 'Landscape & Area Planning', 'Urban planning, parks, gardens'),
(UUID(), '720', 2, '700', 'Architecture', 'Buildings, architectural design'),
(UUID(), '730', 2, '700', 'Sculpture', 'Carving, modeling, statues'),
(UUID(), '740', 2, '700', 'Drawing & Decorative Arts', 'Sketching, calligraphy, design'),
(UUID(), '750', 2, '700', 'Painting', 'Oil painting, watercolor, techniques'),
(UUID(), '760', 2, '700', 'Graphic Arts', 'Printmaking, engraving'),
(UUID(), '770', 2, '700', 'Photography', 'Cameras, techniques, digital photography'),
(UUID(), '780', 2, '700', 'Music', 'Musical theory, instruments, composers'),
(UUID(), '790', 2, '700', 'Sports & Recreation', 'Games, outdoor activities, sports'),

-- Literature (800s)
(UUID(), '810', 2, '800', 'American Literature', 'American poetry, drama, fiction'),
(UUID(), '820', 2, '800', 'English Literature', 'English poetry, drama, fiction'),
(UUID(), '830', 2, '800', 'German Literature', 'German poetry, drama, fiction'),
(UUID(), '840', 2, '800', 'French Literature', 'French poetry, drama, fiction'),
(UUID(), '850', 2, '800', 'Italian Literature', 'Italian poetry, drama, fiction'),
(UUID(), '860', 2, '800', 'Spanish Literature', 'Spanish poetry, drama, fiction'),
(UUID(), '870', 2, '800', 'Latin Literature', 'Classical Latin works'),
(UUID(), '880', 2, '800', 'Greek Literature', 'Classical Greek works'),
(UUID(), '890', 2, '800', 'Other Literatures', 'Asian, African, other world literatures'),

-- History & Geography (900s)
(UUID(), '910', 2, '900', 'Geography & Travel', 'Atlases, travel guides, exploration'),
(UUID(), '920', 2, '900', 'Biography', 'Autobiographies, memoirs, collected biographies'),
(UUID(), '930', 2, '900', 'Ancient World History', 'Egypt, Greece, Rome'),
(UUID(), '940', 2, '900', 'European History', 'European wars, countries'),
(UUID(), '950', 2, '900', 'Asian History', 'China, Japan, India, Middle East'),
(UUID(), '960', 2, '900', 'African History', 'African countries and civilizations'),
(UUID(), '970', 2, '900', 'North American History', 'United States, Canada, Mexico'),
(UUID(), '980', 2, '900', 'South American History', 'Latin American countries'),
(UUID(), '990', 2, '900', 'Other Areas', 'Pacific islands, Australia, polar regions');

-- Insert Common Sections (most frequently used in personal libraries)
INSERT INTO dewey_classifications (id, code, level, parent_code, name, description) VALUES
-- Computer Science sections
(UUID(), '005.1', 3, '005', 'Programming', 'Software development methodologies'),
(UUID(), '005.2', 3, '005', 'Programming for Specific Computers', 'Platform-specific programming'),
(UUID(), '005.7', 3, '005', 'Data in Computer Systems', 'Databases, data structures'),
(UUID(), '006.3', 3, '006', 'Artificial Intelligence', 'Machine learning, neural networks, AI'),
(UUID(), '006.7', 3, '006', 'Multimedia Systems', 'Graphics, animation, video'),

-- Mathematics sections
(UUID(), '512', 3, '510', 'Algebra', 'Linear algebra, abstract algebra'),
(UUID(), '515', 3, '510', 'Analysis', 'Calculus, differential equations'),
(UUID(), '516', 3, '510', 'Geometry', 'Euclidean geometry, topology'),
(UUID(), '519', 3, '510', 'Probability & Statistics', 'Statistical methods, probability theory'),

-- Medicine sections
(UUID(), '612', 3, '610', 'Human Physiology', 'Body functions, organs'),
(UUID(), '613', 3, '610', 'Health & Personal Safety', 'Diet, exercise, wellness'),
(UUID(), '615', 3, '610', 'Pharmacology & Therapeutics', 'Drugs, medicines'),
(UUID(), '616', 3, '610', 'Diseases', 'Pathology, specific diseases'),
(UUID(), '617', 3, '610', 'Surgery', 'Surgical procedures, orthopedics'),
(UUID(), '618', 3, '610', 'Gynecology & Obstetrics', 'Pregnancy, childbirth, women\'s health'),

-- Home & Family sections
(UUID(), '641', 3, '640', 'Food & Cooking', 'Recipes, cooking techniques, cuisines'),
(UUID(), '641.5', 3, '641', 'Cooking', 'Preparation of food, recipes'),
(UUID(), '641.6', 3, '641', 'Cooking Specific Materials', 'Vegetarian, vegan, specific ingredients'),
(UUID(), '642', 3, '640', 'Meals & Table Service', 'Entertaining, table settings'),
(UUID(), '646', 3, '640', 'Sewing & Clothing', 'Fashion, tailoring, textiles'),
(UUID(), '649', 3, '640', 'Child Rearing & Home Care', 'Parenting, child development'),

-- Music sections
(UUID(), '781', 3, '780', 'Music Theory', 'Harmony, composition'),
(UUID(), '782', 3, '780', 'Vocal Music', 'Opera, songs, choral music'),
(UUID(), '783', 3, '780', 'Sacred Music', 'Religious music, hymns'),
(UUID(), '784', 3, '780', 'Instrumental Music', 'Orchestra, chamber music'),
(UUID(), '785', 3, '780', 'Ensembles', 'Bands, orchestras'),
(UUID(), '786', 3, '780', 'Keyboard Instruments', 'Piano, organ'),
(UUID(), '787', 3, '780', 'String Instruments', 'Violin, guitar, cello'),
(UUID(), '788', 3, '780', 'Wind Instruments', 'Flute, trumpet, woodwinds'),

-- Sports & Recreation sections
(UUID(), '791', 3, '790', 'Public Performances', 'Theater, cinema, television'),
(UUID(), '792', 3, '790', 'Stage Presentations', 'Drama, theater production'),
(UUID(), '793', 3, '790', 'Indoor Games & Amusements', 'Board games, card games, puzzles'),
(UUID(), '794', 3, '790', 'Indoor Games of Skill', 'Chess, checkers, video games'),
(UUID(), '795', 3, '790', 'Games of Chance', 'Gambling, lotteries'),
(UUID(), '796', 3, '790', 'Athletic & Outdoor Sports', 'Team sports, individual sports'),
(UUID(), '797', 3, '790', 'Water & Air Sports', 'Swimming, sailing, aviation'),
(UUID(), '798', 3, '790', 'Equestrian Sports', 'Horseback riding, racing'),
(UUID(), '799', 3, '790', 'Fishing & Hunting', 'Angling, shooting'),

-- Fiction sections (most important for personal libraries)
(UUID(), '813', 3, '810', 'American Fiction', 'Novels, short stories by American authors'),
(UUID(), '813.5', 3, '813', 'American Fiction - 1900-1999', '20th century American novels'),
(UUID(), '813.6', 3, '813', 'American Fiction - 2000-', '21st century American novels'),
(UUID(), '823', 3, '820', 'English Fiction', 'Novels, short stories by English authors'),
(UUID(), '823.9', 3, '823', 'English Fiction - 1900-1999', '20th century English novels'),
(UUID(), '833', 3, '830', 'German Fiction', 'Novels, short stories by German authors'),
(UUID(), '843', 3, '840', 'French Fiction', 'Novels, short stories by French authors'),
(UUID(), '853', 3, '850', 'Italian Fiction', 'Novels, short stories by Italian authors'),
(UUID(), '863', 3, '860', 'Spanish Fiction', 'Novels, short stories by Spanish authors'),

-- Poetry sections
(UUID(), '811', 3, '810', 'American Poetry', 'Poetry by American authors'),
(UUID(), '821', 3, '820', 'English Poetry', 'Poetry by English authors'),
(UUID(), '831', 3, '830', 'German Poetry', 'Poetry by German authors'),
(UUID(), '841', 3, '840', 'French Poetry', 'Poetry by French authors'),
(UUID(), '851', 3, '850', 'Italian Poetry', 'Poetry by Italian authors'),
(UUID(), '861', 3, '860', 'Spanish Poetry', 'Poetry by Spanish authors'),

-- Drama sections
(UUID(), '812', 3, '810', 'American Drama', 'Plays by American authors'),
(UUID(), '822', 3, '820', 'English Drama', 'Plays by English authors'),
(UUID(), '832', 3, '830', 'German Drama', 'Plays by German authors'),
(UUID(), '842', 3, '840', 'French Drama', 'Plays by French authors'),
(UUID(), '852', 3, '850', 'Italian Drama', 'Plays by Italian authors'),
(UUID(), '862', 3, '860', 'Spanish Drama', 'Plays by Spanish authors'),

-- Biography sections
(UUID(), '921', 3, '920', 'Biography by Subject', 'Individual biographies'),
(UUID(), '922', 3, '920', 'Collected Biography', 'Multiple biographies'),

-- History sections
(UUID(), '941', 3, '940', 'British Isles', 'History of England, Scotland, Wales, Ireland'),
(UUID(), '943', 3, '940', 'Central Europe', 'History of Germany, Austria'),
(UUID(), '944', 3, '940', 'France', 'History of France'),
(UUID(), '945', 3, '940', 'Italy', 'History of Italy'),
(UUID(), '946', 3, '940', 'Spain & Portugal', 'History of Iberian peninsula'),
(UUID(), '947', 3, '940', 'Russia', 'History of Russia and Soviet Union'),
(UUID(), '948', 3, '940', 'Scandinavia', 'History of Nordic countries'),
(UUID(), '949', 3, '940', 'Other European Countries', 'Greece, Balkans, etc.'),
(UUID(), '951', 3, '950', 'China', 'History of China'),
(UUID(), '952', 3, '950', 'Japan', 'History of Japan'),
(UUID(), '953', 3, '950', 'Arabian Peninsula', 'History of Middle East'),
(UUID(), '954', 3, '950', 'India', 'History of India and South Asia'),
(UUID(), '973', 3, '970', 'United States', 'American history');
