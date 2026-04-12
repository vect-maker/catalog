import { z } from 'zod';

// Generic Pagination Wrapper

export interface PaginatedResponse<T> {
    page_size: number;
    page: number;
    is_last: boolean;
    items: T[];
}

export function createPaginatedSchema<T>(
    itemSchema: z.ZodType<T>
): z.ZodType<PaginatedResponse<T>> {
    return z.object({
        page_size: z.number().int().nonnegative(),
        page: z.number().int().nonnegative(),
        is_last: z.boolean(),
        items: z.array(itemSchema),
    });
}

// Generics 
export const CreateResponseIdSchema = z.object({
    id: z.uuidv7()
});

// Auth
export const AuthenticateUserSchema = z.object({
    name: z.string(),
    password: z.string()
});

export const AuthenticationToken = z.object({
    token: z.string()
});

// Product
export const ProductSchema = z.object({
    id: z.uuidv7(),
    title: z.string(),
    description: z.string().nullable(),
    price: z.number(),
    images: z.array(z.string())
});

export type Product = z.infer<typeof ProductSchema>;

export const ProductCreateSchema = z.object({
    title: z.string().min(4).max(40),
    description: z.string().min(4).max(500).nullable(),
    price: z.number().positive(),
});

// Paginated Entities
export const PaginatedProductsSchema = createPaginatedSchema(ProductSchema);

export type PaginatedProducts = z.infer<typeof PaginatedProductsSchema>;