// Define the recipe data required by the detail page.
export interface Recipe {
  id: string;
  title: string;
  description: string;
  ingredients: { name: string; amount: string }[];
  steps: { number: number; text: string; image_urls: string[] }[];
  tips: string;
  cover_image_urls: string[];
  author: string;
  author_url: string;
  url: string;
}
