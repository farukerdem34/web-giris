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

