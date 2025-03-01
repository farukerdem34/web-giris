-- Initialize the schema for reservations
CREATE TABLE IF NOT EXISTS reservations (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    date TIMESTAMP NOT NULL
);

-- Initialize the schema for events
CREATE TABLE IF NOT EXISTS events (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    date TIMESTAMP NOT NULL,
    venue VARCHAR(255) NOT NULL
);

-- Initialize the schema for tickets
CREATE TABLE IF NOT EXISTS tickets (
    id SERIAL PRIMARY KEY,
    event_id INT REFERENCES events(id) ON DELETE CASCADE,
    seat_number VARCHAR(50) NOT NULL,
    price DECIMAL(10, 2) NOT NULL,
    status VARCHAR(50) NOT NULL CHECK (status IN ('available', 'sold', 'reserved'))
);

-- Add some example data
INSERT INTO events (name, date, venue) VALUES 
('Music Concert', '2025-05-01 19:00:00', 'Arena Hall'),
('Theater Play', '2025-06-15 20:00:00', 'City Theater');

INSERT INTO reservations (name, description, date) VALUES
('VIP Reservation', 'VIP ticket for Music Concert', '2025-05-01 18:00:00'),
('Regular Reservation', 'Regular ticket for Theater Play', '2025-06-15 19:30:00');

INSERT INTO tickets (event_id, seat_number, price, status) VALUES
(1, 'A1', 100.00, 'available'),
(1, 'A2', 100.00, 'sold'),
(2, 'B1', 50.00, 'reserved'),
(2, 'B2', 50.00, 'available');

