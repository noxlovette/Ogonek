import fs from "fs/promises";
import path from "path";

const OPENAPI_PATH = "./openapi.json";
const MOCK_DIR = "./src/routes/api/mock";

/**
 * Generates TypeScript interface from OpenAPI schema
 */
function generateTypeFromSchema(schema, name = "Response") {
  if (!schema) return "any";

  if (schema.type === "object" && schema.properties) {
    const props = Object.entries(schema.properties)
      .map(([key, prop]) => {
        const optional = !schema.required?.includes(key) ? "?" : "";
        const type = generateTypeFromSchema(prop);
        return `  ${key}${optional}: ${type};`;
      })
      .join("\n");

    return `{\n${props}\n}`;
  }

  if (schema.type === "array") {
    return `${generateTypeFromSchema(schema.items)}[]`;
  }

  const typeMap = {
    string: "string",
    number: "number",
    integer: "number",
    boolean: "boolean",
    object: "Record<string, any>",
  };

  return typeMap[schema.type] || "any";
}

/**
 * Generates mock data based on OpenAPI schema
 */
function generateMockData(schema) {
  if (!schema) return null;

  if (schema.example) return JSON.stringify(schema.example, null, 2);

  if (schema.type === "object" && schema.properties) {
    const mock = {};
    Object.entries(schema.properties).forEach(([key, prop]) => {
      if (schema.required?.includes(key) || Math.random() > 0.3) {
        mock[key] = generateMockValue(prop);
      }
    });
    return JSON.stringify(mock, null, 2);
  }

  if (schema.type === "array") {
    const items = Array(2)
      .fill(null)
      .map(() => generateMockValue(schema.items));
    return JSON.stringify(items, null, 2);
  }

  return JSON.stringify(generateMockValue(schema), null, 2);
}

function generateMockValue(schema) {
  if (!schema) return null;

  if (schema.example !== undefined) return schema.example;

  const mockMap = {
    string:
      schema.format === "email"
        ? "user@example.com"
        : schema.format === "date-time"
          ? new Date().toISOString()
          : schema.format === "uuid"
            ? "uuid-example-123"
            : "example string",
    number: 42,
    integer: 42,
    boolean: true,
    object: {},
  };

  return mockMap[schema.type] || null;
}

/**
 * Generates HTTP status response template
 */
function generateStatusResponse(status, responseSchema, useJson = true) {
  const statusMessages = {
    200: "OK",
    201: "Created",
    204: "No Content",
    400: "Bad Request",
    401: "Unauthorized",
    403: "Forbidden",
    404: "Not Found",
    422: "Unprocessable Entity",
    500: "Internal Server Error",
  };

  const message = statusMessages[status] || "Response";

  if (status === 204) {
    return `    case 204:
      return new Response(null, { status: 204 });`;
  }

  const mockData = responseSchema ? generateMockData(responseSchema) : "null";

  if (useJson) {
    return `    case ${status}:
      return json(${mockData});`;
  } else {
    return `    case ${status}:
      return new Response(JSON.stringify(${mockData}), {
        headers: { "Content-Type": "application/json" },
      });`;
  }
}

/**
 * Generates the modern Svelte API route file content
 */
function generateRouteFile(method, path, operation, allMethods) {
  const methodUpper = method.toUpperCase();
  const operationId =
    operation.operationId || `${method}${path.replace(/[^a-zA-Z0-9]/g, "")}`;

  // Extract possible response codes
  const responses = Object.keys(operation.responses || {}).filter(
    (code) => code !== "default",
  );
  const defaultResponse = operation.responses?.default;

  // Decide whether to use json() or new Response() based on complexity
  const useJson = responses.length <= 2 && !operation.requestBody;

  // Generate response cases
  const responseCases = responses
    .map((status) => {
      const responseSchema =
        operation.responses[status]?.content?.["application/json"]?.schema;
      return generateStatusResponse(parseInt(status), responseSchema, useJson);
    })
    .join("\n\n");

  const defaultCase = defaultResponse
    ? `\n    default:\n      return json({ error: "Unexpected response" }, { status: 500 });`
    : `\n    default:\n      return json({ error: "Not implemented" }, { status: 501 });`;

  // Extract request body schema for POST/PUT/PATCH
  const requestBodySchema =
    operation.requestBody?.content?.["application/json"]?.schema;
  const requestBodyType = requestBodySchema
    ? generateTypeFromSchema(requestBodySchema, "RequestBody")
    : null;

  // Generate parameter types
  const pathParams = operation.parameters?.filter((p) => p.in === "path") || [];
  const queryParams =
    operation.parameters?.filter((p) => p.in === "query") || [];

  // Build imports
  const imports = ["json"];
  const typeImports = ["RequestHandler"];

  // Add logger import if POST/PUT/PATCH
  const needsLogger = ["post", "put", "patch"].includes(method);
  const loggerImport = needsLogger ? 'import logger from "$lib/logger";' : "";

  // Generate type definitions
  const typeDefinitions = requestBodyType
    ? `type RequestBody = ${requestBodyType};\n`
    : "";

  // Generate handler function
  const handlerParams =
    requestBodyType || pathParams.length || queryParams.length
      ? "{ request, params, url }"
      : "{}";

  // Generate body parsing
  const bodyParsing = requestBodyType
    ? `  const body = await request.json();\n  logger.info("${methodUpper} ${path} with body:", body);\n`
    : "";

  // Generate parameter comments
  const paramComments = [
    pathParams.length
      ? `  // Path params: ${pathParams.map((p) => p.name).join(", ")}`
      : "",
    queryParams.length
      ? `  // Query params: ${queryParams.map((p) => p.name).join(", ")}`
      : "",
  ]
    .filter(Boolean)
    .join("\n");

  const paramsSection = paramComments ? `${paramComments}\n` : "";

  return `${loggerImport}
import { ${imports.join(", ")} } from "@sveltejs/kit";
import type { ${typeImports.join(", ")} } from "./$types";

// Generated mock for ${methodUpper} ${path}
// Operation: ${operation.summary || operationId}
${operation.description ? `// ${operation.description}` : ""}

${typeDefinitions}export const ${methodUpper}: RequestHandler = async (${handlerParams}) => {
${bodyParsing}${paramsSection}  // Mock response selector - customize this logic
  const mockResponse = url?.searchParams.get('mock_status') || '${responses[0] || "200"}';
  
  // Return mock based on requested status
  switch (parseInt(mockResponse)) {
${responseCases}${defaultCase}
  }
};`;
}

/**
 * Generates a consolidated route file with all methods
 */
function generateConsolidatedRouteFile(apiPath, pathItem) {
  const methods = Object.keys(pathItem).filter((method) =>
    ["get", "post", "put", "patch", "delete"].includes(method),
  );

  if (methods.length === 0) return null;

  const allHandlers = methods.map((method) =>
    generateRouteFile(method, apiPath, pathItem[method], methods),
  );

  // Extract common imports and combine
  const hasLogger = methods.some((method) =>
    ["post", "put", "patch"].includes(method),
  );
  const loggerImport = hasLogger ? 'import logger from "$lib/logger";' : "";

  // Get all unique type definitions
  const allTypeDefinitions = new Set();
  methods.forEach((method) => {
    const operation = pathItem[method];
    const requestBodySchema =
      operation.requestBody?.content?.["application/json"]?.schema;
    if (requestBodySchema) {
      const typeDef = generateTypeFromSchema(
        requestBodySchema,
        `${method.toUpperCase()}RequestBody`,
      );
      allTypeDefinitions.add(
        `type ${method.toUpperCase()}RequestBody = ${typeDef};`,
      );
    }
  });

  const typeDefinitionsSection =
    allTypeDefinitions.size > 0
      ? `\n${Array.from(allTypeDefinitions).join("\n")}\n`
      : "";

  // Generate individual handlers
  const handlers = methods
    .map((method) => {
      const operation = pathItem[method];
      const methodUpper = method.toUpperCase();
      const operationId =
        operation.operationId ||
        `${method}${apiPath.replace(/[^a-zA-Z0-9]/g, "")}`;

      // Extract response info
      const responses = Object.keys(operation.responses || {}).filter(
        (code) => code !== "default",
      );
      const requestBodySchema =
        operation.requestBody?.content?.["application/json"]?.schema;
      const hasBody =
        requestBodySchema && ["post", "put", "patch"].includes(method);
      const pathParams =
        operation.parameters?.filter((p) => p.in === "path") || [];
      const queryParams =
        operation.parameters?.filter((p) => p.in === "query") || [];

      // Handler parameters
      const handlerParams =
        hasBody || pathParams.length || queryParams.length
          ? "{ request, params, url }"
          : "{}";

      // Body parsing
      const bodyParsing = hasBody
        ? `  const body = await request.json();\n  logger.info("${methodUpper} ${apiPath} with body:", body);\n`
        : "";

      // Parameter comments
      const paramComments = [
        pathParams.length
          ? `  // Path params: ${pathParams.map((p) => p.name).join(", ")}`
          : "",
        queryParams.length
          ? `  // Query params: ${queryParams.map((p) => p.name).join(", ")}`
          : "",
      ]
        .filter(Boolean)
        .join("\n");

      const paramsSection = paramComments ? `${paramComments}\n` : "";

      // Generate mock response
      const mockData = responses[0]
        ? operation.responses[responses[0]]?.content?.["application/json"]
            ?.schema
          ? generateMockData(
              operation.responses[responses[0]].content["application/json"]
                .schema,
            )
          : "null"
        : '{ message: "Success" }';

      return `export const ${methodUpper}: RequestHandler = async (${handlerParams}) => {
${bodyParsing}${paramsSection}  // ${operation.summary || operationId}
  ${operation.description ? `// ${operation.description}` : ""}
  
  return json(${mockData});
};`;
    })
    .join("\n\n");

  return `${loggerImport}
import { json } from "@sveltejs/kit";
import type { RequestHandler } from "./$types";${typeDefinitionsSection}
${handlers}`;
}

/**
 * Converts OpenAPI path to SvelteKit route path and strips /api/v1
 */
function convertToSvelteRoute(apiPath) {
  // Strip /api/v1 prefix since we're already in api/mock
  const cleanPath = apiPath.replace(/^\/api\/v1/, "") || "/";
  return cleanPath.replace(/{([^}]+)}/g, "[$1]");
}

/**
 * Recursively finds all +server.ts files in a directory
 */
async function findExistingRoutes(dir, baseDir = dir) {
  const routes = [];

  try {
    const entries = await fs.readdir(dir, { withFileTypes: true });

    for (const entry of entries) {
      const fullPath = path.join(dir, entry.name);

      if (entry.isDirectory()) {
        routes.push(...(await findExistingRoutes(fullPath, baseDir)));
      } else if (entry.name === "+server.ts") {
        const relativePath = path.relative(baseDir, path.dirname(fullPath));
        routes.push(relativePath === "" ? "/" : `/${relativePath}`);
      }
    }
  } catch (error) {
    // Directory doesn't exist, return empty array
  }

  return routes;
}

async function generateMocks() {
  try {
    console.log("🔍 Reading OpenAPI spec...");
    const jsonContent = await fs.readFile(OPENAPI_PATH, "utf8");
    const spec = JSON.parse(jsonContent);

    if (!spec.paths) {
      console.log("❌ No paths found in OpenAPI spec");
      return;
    }

    // Ensure mock directory exists
    await fs.mkdir(MOCK_DIR, { recursive: true });

    // Find existing routes for cleanup
    const existingRoutes = await findExistingRoutes(MOCK_DIR);
    const validRoutes = new Set();

    // Collect all valid routes from spec
    for (const [apiPath] of Object.entries(spec.paths)) {
      const cleanPath = apiPath.replace(/^\/api\/v1/, "") || "/";
      const svelteRoute = cleanPath.replace(/{([^}]+)}/g, "[$1]");
      validRoutes.add(svelteRoute);
    }

    // Remove orphaned routes
    let removed = 0;
    for (const existingRoute of existingRoutes) {
      if (!validRoutes.has(existingRoute)) {
        const routeDir = path.join(
          MOCK_DIR,
          existingRoute === "/" ? "" : existingRoute,
        );
        const serverFile = path.join(routeDir, "+server.ts");

        try {
          await fs.unlink(serverFile);
          console.log(`🗑️  Removed orphaned: ${existingRoute}`);
          removed++;

          // Try to remove empty directory
          try {
            await fs.rmdir(routeDir);
          } catch {
            // Directory not empty or doesn't exist, that's fine
          }
        } catch (error) {
          console.warn(
            `⚠️  Failed to remove ${existingRoute}: ${error.message}`,
          );
        }
      }
    }

    let generated = 0;
    let skipped = 0;

    for (const [apiPath, pathItem] of Object.entries(spec.paths)) {
      const svelteRoute = convertToSvelteRoute(apiPath);
      const routeDir = path.join(MOCK_DIR, svelteRoute);

      // Create directory structure
      await fs.mkdir(routeDir, { recursive: true });

      const filename = "+server.ts";
      const filepath = path.join(routeDir, filename);

      // Skip if file exists
      try {
        await fs.access(filepath);
        console.log(`⏭️  Skipping existing: ${apiPath}`);
        skipped++;
        continue;
      } catch {
        // File doesn't exist, proceed
      }

      console.log(`✨ Generating: ${apiPath}`);

      const content = generateConsolidatedRouteFile(apiPath, pathItem);
      if (content) {
        await fs.writeFile(filepath, content);
        generated++;
      }
    }

    console.log(`\n🎉 Generation complete!`);
    console.log(`✅ Generated: ${generated} routes`);
    console.log(`⏭️  Skipped: ${skipped} existing routes`);
    console.log(`🗑️  Removed: ${removed} orphaned routes`);
    console.log(
      `\n💡 Tip: Use ?mock_status=404 in your URLs to test different responses`,
    );
  } catch (error) {
    console.error("❌ Error:", error.message);
    process.exit(1);
  }
}

// Handle command line usage
if (import.meta.url === `file://${process.argv[1]}`) {
  generateMocks();
}

export { generateMocks };
