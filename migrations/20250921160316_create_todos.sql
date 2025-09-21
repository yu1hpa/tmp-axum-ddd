CREATE TABLE todos (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    title TEXT NOT NULL,
    description TEXT,
    completed BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT (now() AT TIME ZONE 'Asia/Tokyo') NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT (now() AT TIME ZONE 'Asia/Tokyo') NOT NULL
);