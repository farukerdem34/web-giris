-- Create Events table
CREATE TABLE IF NOT EXISTS events (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    date VARCHAR(255) NOT NULL,
    venue VARCHAR(255) NOT NULL
);

-- Create Tickets table
CREATE TABLE IF NOT EXISTS tickets (
    id UUID PRIMARY KEY,
    event_id UUID NOT NULL,
    seat_number INT NOT NULL,
    price INT NOT NULL,
    status VARCHAR(20) CHECK (status IN ('Available', 'Reserved', 'Sold')) NOT NULL,
    FOREIGN KEY (event_id) REFERENCES events(id) ON DELETE CASCADE
);

-- Create Reservations table
CREATE TABLE IF NOT EXISTS reservations (
    id UUID PRIMARY KEY,
    ticket_id UUID NOT NULL,
    customer_name VARCHAR(255) NOT NULL,
    reservation_date VARCHAR(255) NOT NULL,
    status VARCHAR(20) CHECK (status IN ('Pending', 'Confirmed', 'Cancelled')) NOT NULL,
    FOREIGN KEY (ticket_id) REFERENCES tickets(id) ON DELETE CASCADE
);

-- Example Inserts for testing

-- Insert sample events
INSERT INTO events (id, name, date, venue) VALUES
('c9d7d951-f343-4c9d-82d0-d2ef3d68b350', 'Rock Concert', '2025-06-01', 'Stadium Arena'),
('66d79560-1eaf-42e5-89b7-0c62f4658f83', 'Tech Conference', '2025-07-15', 'Conference Hall');

-- Insert sample tickets
INSERT INTO tickets (id, event_id, seat_number, price, status) VALUES
('3ffbfe47-4e5b-45fa-9c1f-8f350be02bdb', 'c9d7d951-f343-4c9d-82d0-d2ef3d68b350', 1, 100, 'Available'),
('6ed22c79-d2f5-4d3b-8fa7-12c1bb02062b', '66d79560-1eaf-42e5-89b7-0c62f4658f83', 2, 200, 'Reserved'),
('7a1b0adf-4583-4a0a-b8ca-585902495a01', 'c9d7d951-f343-4c9d-82d0-d2ef3d68b350', 3, 150, 'Sold');

-- Insert sample reservations
INSERT INTO reservations (id, ticket_id, customer_name, reservation_date, status) VALUES
('4b07a70c-c0b6-45cf-bb7f-1048a935d1b0', '3ffbfe47-4e5b-45fa-9c1f-8f350be02bdb', 'Alice Johnson', '2025-05-10', 'Pending'),
('3b6e19b4-1e3f-4ab1-8d9e-b527c9982e24', '6ed22c79-d2f5-4d3b-8fa7-12c1bb02062b', 'Bob Smith', '2025-06-01', 'Confirmed'),
('cf19f679-cf32-4707-98de-582da1a567c3', '7a1b0adf-4583-4a0a-b8ca-585902495a01', 'Charlie Lee', '2025-03-10', 'Cancelled');


