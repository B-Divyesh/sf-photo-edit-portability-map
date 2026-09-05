CREATE TABLE Adobe_images (
  id INTEGER PRIMARY KEY,
  rating INTEGER,
  captureTime TEXT,
  developSettings TEXT
);
INSERT INTO Adobe_images (rating, captureTime, developSettings) VALUES
  (5, '2024-04-12T06:41:00', 'sample-develop-recipe-a'),
  (3, '2024-04-12T12:13:00', 'sample-develop-recipe-b'),
  (0, '2024-04-13T16:08:00', 'sample-develop-recipe-c');
