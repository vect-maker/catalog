import { z } from 'zod';

const API_BASE_URL = import.meta.env.VITE_API_URL;

export const ProductSchema = z.object({
  id: z.number(),
  title: z.string(),
  description: z.string(),
  price: z.number(),
  image_id: z.number().nullable()
});

export type Product = z.infer<typeof ProductSchema>;

export const ProductListSchema = z.array(ProductSchema);
export type ProductList = z.infer<typeof ProductListSchema>;


export const ProductCreateSchema = z.object({
  title: z.string().min(4).max(40),
  description: z.string().min(4).max(500),
  price: z.number().positive(),
})

export const CreateResponseSchema = z.object({
  id: z.number()
})


export const getImageUrl = (imageId: number) => {
  return `${API_BASE_URL}/images/${imageId}`
}


export async function apiFetch<T>(path: string, schema: z.ZodSchema<T>): Promise<T> {
  const response = await fetch(`${API_BASE_URL}${path}`);

  if (!response.ok) throw new Error(`HTTP error! status: ${response.status}`);

  const data = await response.json();

  return schema.parse(data);
}

export const createProduct = async (formData: unknown) => {
  const result = ProductCreateSchema.safeParse(formData);

  if (!result.success) {
    console.error("Validation failed:", z.prettifyError(result.error));
    throw new Error(`Form data is not correct`)
  }

  const response = await fetch(`${API_BASE_URL}/products`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(result.data),
  });

  if (!response.ok) {
    throw new Error(`Server error: ${response.status}`);
  }

  return CreateResponseSchema.parse(await response.json())

}

export const addImageToProduct = async (product_id: number, file: File) => {
  const formData = new FormData();

  formData.append('image', file);

  const response = await fetch(`${API_BASE_URL}/products/${product_id}/images`, {
    method: 'POST',
    body: formData,
  });

  if (!response.ok) throw new Error("Upload failed");

  return CreateResponseSchema.parse(await response.json())

}