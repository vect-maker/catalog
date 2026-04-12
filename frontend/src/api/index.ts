import { useAuthStore } from '../stores/useAuthStore';
import { AuthenticateUserSchema, AuthenticationToken, CreateResponseIdSchema, PaginatedProductsSchema, ProductCreateSchema, ProductSchema } from './schemas';
import { z } from 'zod';

const API_BASE_URL = import.meta.env.VITE_API_URL;

// handlers 
export const getImageUrl = (imageId: string) => {
  return `${API_BASE_URL}/images/${imageId}`
}


export async function apiFetch<T>(path: string, schema: z.ZodSchema<T>): Promise<T> {
  const response = await fetch(`${API_BASE_URL}${path}`);

  if (!response.ok) throw new Error(`HTTP error! status: ${response.status}`);

  const data = await response.json();

  return schema.parse(data);
}

export const checkIfBackendReady = async () => {
  const response = await fetch(`${API_BASE_URL}/health/ready`);
  return response.ok
}

export const getPaginatedProducts = async (page: number = 0, q: string | null = null) => {
  const url = new URL(`${API_BASE_URL}/products`);
  url.searchParams.append('page', String(page));

  if (q) {
    url.searchParams.append('q', q);
  }

  const response = await fetch(url);

  if (!response.ok) throw new Error(`HTTP error! status: ${response.status}`);

  const data = await response.json();

  return PaginatedProductsSchema.parse(data);
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

  return CreateResponseIdSchema.parse(await response.json())

}

export const deleteProduct = async (product_id: string) => {
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

export const getProduct = async (product_id: string) => {

  const response = await fetch(`${API_BASE_URL}/products/${product_id}`)

  if (!response.ok) return null

  return ProductSchema.parse(await response.json())
}

export const addImageToProduct = async (product_id: string, file: File) => {
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

  return CreateResponseIdSchema.parse(await response.json())

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