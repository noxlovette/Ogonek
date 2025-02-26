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

-- Consider adding a counter cache for pagination efficiency
ALTER TABLE "user" ADD COLUMN IF NOT EXISTS lessons_count INTEGER DEFAULT 0;
ALTER TABLE "user" ADD COLUMN IF NOT EXISTS tasks_count INTEGER DEFAULT 0;

-- Create a function to update the lesson counts
CREATE OR REPLACE FUNCTION update_lesson_count()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        -- Update both assignee and creator counts
        UPDATE "user" SET lessons_count = lessons_count + 1 WHERE id = NEW.assignee;
        IF NEW.assignee != NEW.created_by THEN
            UPDATE "user" SET lessons_count = lessons_count + 1 WHERE id = NEW.created_by;
        END IF;
    ELSIF TG_OP = 'DELETE' THEN
        -- Update both assignee and creator counts
        UPDATE "user" SET lessons_count = lessons_count - 1 WHERE id = OLD.assignee;
        IF OLD.assignee != OLD.created_by THEN
            UPDATE "user" SET lessons_count = lessons_count - 1 WHERE id = OLD.created_by;
        END IF;
    END IF;
    
    RETURN NULL;
END;
$$ language 'plpgsql';

-- Create trigger for lessons
CREATE TRIGGER update_lesson_count_trigger
AFTER INSERT OR DELETE ON lessons
FOR EACH ROW EXECUTE FUNCTION update_lesson_count();

-- Create a function to update the task counts
CREATE OR REPLACE FUNCTION update_task_count()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        -- Update both assignee and creator counts
        UPDATE "user" SET tasks_count = tasks_count + 1 WHERE id = NEW.assignee;
        IF NEW.assignee != NEW.created_by THEN
            UPDATE "user" SET tasks_count = tasks_count + 1 WHERE id = NEW.created_by;
        END IF;
    ELSIF TG_OP = 'DELETE' THEN
        -- Update both assignee and creator counts
        UPDATE "user" SET tasks_count = tasks_count - 1 WHERE id = OLD.assignee;
        IF OLD.assignee != OLD.created_by THEN
            UPDATE "user" SET tasks_count = tasks_count - 1 WHERE id = OLD.created_by;
        END IF;
    END IF;
    
    RETURN NULL;
END;
$$ language 'plpgsql';

-- Create trigger for tasks
CREATE TRIGGER update_task_count_trigger
AFTER INSERT OR DELETE ON tasks
FOR EACH ROW EXECUTE FUNCTION update_task_count();

-- Populate the initial counts (run once after migration)
UPDATE "user" u SET 
    lessons_count = (
        SELECT COUNT(*) FROM lessons l 
        WHERE l.assignee = u.id OR l.created_by = u.id
    ),
    tasks_count = (
        SELECT COUNT(*) FROM tasks t 
        WHERE t.assignee = u.id OR t.created_by = u.id
    );