-- Optimized indices for lesson pagination
CREATE INDEX IF NOT EXISTS idx_lessons_created_at_id ON lessons(created_at DESC, id);
CREATE INDEX IF NOT EXISTS idx_lessons_assignee_created_at ON lessons(assignee, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_lessons_created_by_created_at ON lessons(created_by, created_at DESC);

-- Optimized indices for task pagination
CREATE INDEX IF NOT EXISTS idx_tasks_due_date_id ON tasks(due_date DESC, id);
CREATE INDEX IF NOT EXISTS idx_tasks_assignee_due_date ON tasks(assignee, due_date DESC);
CREATE INDEX IF NOT EXISTS idx_tasks_created_by_due_date ON tasks(created_by, due_date DESC);

-- Optimized indices for card progress pagination (for learning/flashcards)
CREATE INDEX IF NOT EXISTS idx_card_progress_user_due_date ON card_progress(user_id, due_date);

-- Add updated_at trigger if not already present
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';
