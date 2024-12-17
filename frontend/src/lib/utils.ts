export function formatDate (unformatted: string) {
    const date = new Date(unformatted);

const formattedDate = date.toLocaleDateString('en-GB', {
    month: 'short',
    day: 'numeric',
    year: 'numeric'
});
  return formattedDate;
}