import { z } from 'zod';
import { useAuthStore } from './stores/useAuthStore';

const API_BASE_URL = import.meta.env.VITE_API_URL;

export const ProductSchema = z.object({
  id: z.number(),
  title: z.string(),
  description: z.string(),
  price: z.number(),
  images: z.array(z.number())
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

export const AuthenticateUserSchema = z.object({
  name: z.string(),
  password: z.string()
})

export const AuthenticationToken = z.object({
  token: z.string()
})


// handlers 
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
  const authStore = useAuthStore();


  if (!result.success) {
    console.error("Validation failed:", z.prettifyError(result.error));
    throw new Error(`Form data is not correct`)
  }

  const response = await fetch(`${API_BASE_URL}/products`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'Authorization': `Bearer ${authStore.token}`
    },
    body: JSON.stringify(result.data),
  });

  if (!response.ok) {
    throw new Error(`Server error: ${response.status}`);
  }

  return CreateResponseSchema.parse(await response.json())

}

export const deleteProduct = async (product_id: number) => {
  const authStore = useAuthStore();

  const response = await fetch(`${API_BASE_URL}/products/${product_id}`, {
    method: 'DELETE',
    headers: {
      'Authorization': `Bearer ${authStore.token}`
    },
  });

  if (!response.ok) {
    throw new Error(`Server error: ${response.status}`);
  }
}

export const addImageToProduct = async (product_id: number, file: File) => {
  const formData = new FormData();
  const authStore = useAuthStore();

  formData.append('image', file);

  const response = await fetch(`${API_BASE_URL}/products/${product_id}/images`, {
    method: 'POST',
    headers: {
      'Authorization': `Bearer ${authStore.token}`
    },
    body: formData,
  });

  if (!response.ok) throw new Error("Upload failed");

  return CreateResponseSchema.parse(await response.json())

}

export const authenticate = async (formData: unknown) => {
  const payload = AuthenticateUserSchema.safeParse(formData);

  if (!payload.success) {
    console.error("Validation failed:", z.prettifyError(payload.error));
    throw new Error(`Form data is not correct`)
  }

  const response = await fetch(`${API_BASE_URL}/users/authenticate`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(payload.data),
  });

  if (!response.ok) {
    throw new Error(`Server error: ${response.status}`);
  }


  return AuthenticationToken.parse(await response.json())
}