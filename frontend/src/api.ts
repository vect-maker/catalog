import { z } from 'zod';

const API_BASE_URL = import.meta.env.VITE_API_URL;

export const ProductSchema = z.object({
  id: z.number(),
  title: z.string(),
  description: z.string(),
  price: z.number(),
  image_id: z.number()
});

export type Product = z.infer<typeof ProductSchema>;

export const ProductListSchema = z.array(ProductSchema);
export type ProductList = z.infer<typeof ProductListSchema>;


export const  getImageUrl = (imageId: number)=>{
  return `${API_BASE_URL}/images/${imageId}`
} 


export async function apiFetch<T>(path: string, schema: z.ZodSchema<T>): Promise<T> {
  const response = await fetch(`${API_BASE_URL}${path}`);
  
  if (!response.ok) throw new Error(`HTTP error! status: ${response.status}`);
  
  const data = await response.json();
  
  return schema.parse(data);
}

