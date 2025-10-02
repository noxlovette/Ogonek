#!/usr/bin/env node

import { readFileSync, writeFileSync } from "fs";

class RouteGenerator {
  constructor(apiBase = "/axum", stripPrefix = "/api/v1") {
    this.apiBase = apiBase;
    this.stripPrefix = stripPrefix;
    this.routes = {};
  }

  generate(specPath, outputPath = "src/lib/types/routes.ts") {
    const spec = JSON.parse(readFileSync(specPath, "utf-8"));
    this.processRoutes(spec.paths);
    this.writeRoutes(outputPath);
    console.log(`🛣️  Generated routes at ${outputPath}`);
  }

  processRoutes(paths) {
    Object.entries(paths).forEach(([path, methods]) => {
      const cleanPath = this.cleanPath(path);
      const segments = cleanPath.split("/").filter(Boolean);

      if (segments.length === 0) return;

      const group = this.extractGroupName(segments);
      const operation = this.determineOperation(cleanPath, segments, methods);

      if (!this.routes[group]) {
        this.routes[group] = {};
      }

      // Process each HTTP method to create appropriate route names
      Object.entries(methods).forEach(([method, spec]) => {
        if (!["get", "post", "put", "delete", "patch"].includes(method)) return;

        const routeName = this.getRouteName(
          operation,
          method,
          spec.operationId,
          cleanPath,
        );

        // Don't overwrite existing routes
        if (!this.routes[group][routeName]) {
          this.routes[group][routeName] = this.createRouteFunction(
            cleanPath,
            spec.parameters,
          );
        }
      });
    });
  }

  extractGroupName(segments) {
    const firstSegment = segments[0];
    if (firstSegment.includes("{") && firstSegment.includes("}")) {
      return firstSegment.split("{")[0];
    }
    return firstSegment;
  }

  cleanPath(path) {
    return path.startsWith(this.stripPrefix)
      ? path.slice(this.stripPrefix.length)
      : path;
  }

  determineOperation(path, segments) {
    if (segments.length === 1) {
      return "root";
    }

    const lastSegment = segments[segments.length - 1];
    if (lastSegment.startsWith("{") && lastSegment.endsWith("}")) {
      return "single";
    }

    if (segments.length >= 3) {
      const secondLast = segments[segments.length - 2];
      if (secondLast.startsWith("{") && secondLast.endsWith("}")) {
        return segments[segments.length - 1];
      }
    }

    return segments[segments.length - 1];
  }

  getRouteName(operation, method, operationId) {
    // Use operationId mapping first if available
    if (operationId) {
      const mapped = this.operationIdToRouteName(operationId);
      return mapped;
    }

    // Handle root operations (e.g., /decks, /tasks, /lessons)
    if (operation === "root") {
      if (method === "get") return "all";
      if (method === "post") return "new"; // This is the key fix!
      return method;
    }

    // Handle single resource operations (e.g., /decks/{id})
    if (operation === "single") {
      const methodMap = {
        get: "single",
        put: "update",
        patch: "update",
        delete: "delete",
        post: "update", // Some APIs use POST for updates
      };
      return methodMap[method] || "single";
    }

    // For other operations, use the operation name directly
    return operation;
  }

  operationIdToRouteName(operationId) {
    const operationMap = {
      // Explicit mappings
      list_decks: "all",
      create_deck: "new",
      get_deck: "single",
      update_deck: "update",
      delete_deck: "delete",
      duplicate_deck: "duplicate",

      list_lessons: "all",
      create_lesson: "new",
      get_lesson: "single",
      update_lesson: "update",
      delete_lesson: "delete",

      list_tasks: "all",
      create_task: "new",
      get_task: "single",
      update_task: "update",
      delete_task: "delete",

      // CamelCase versions
      listDecks: "all",
      createDeck: "new",
      getDeck: "single",
      updateDeck: "update",
      deleteDeck: "delete",
      duplicateDeck: "duplicate",

      listLessons: "all",
      createLesson: "new",
      getLesson: "single",
      updateLesson: "update",
      deleteLesson: "delete",

      listTasks: "all",
      createTask: "new",
      getTask: "single",
      updateTask: "update",
      deleteTask: "delete",
    };

    if (operationMap[operationId]) {
      return operationMap[operationId];
    }

    // Fallback: convert and clean up
    return operationId
      .replace(/([A-Z])/g, "_$1")
      .toLowerCase()
      .replace(/^_/, "")
      .replace(/^(list|get|fetch)_/, "")
      .replace(/^create_/, "new_")
      .replace(/_deck$/, "")
      .replace(/_lesson$/, "")
      .replace(/_task$/, "")
      .replace(/^new_$/, "new");
  }

  createRouteFunction(path, parameters = []) {
    const pathParams = this.extractPathParams(path);
    const queryParams = parameters?.filter((p) => p.in === "query") || [];

    if (pathParams.length === 0 && queryParams.length === 0) {
      return `() => \`\${API_BASE}${path}\``;
    }

    if (pathParams.length > 0 && queryParams.length === 0) {
      const paramsType = pathParams.map((p) => `${p}: string`).join("; ");
      const pathWithParams = this.replacePathParams(path);
      return `(params: { ${paramsType} }) => \`\${API_BASE}${pathWithParams.replace(/\$\{(\w+)\}/g, "${params.$1}")}\``;
    }

    if (queryParams.length > 0) {
      const pathWithParams = this.replacePathParams(path);

      return this.generateQueryParamFunction(
        pathWithParams,
        pathParams,
        queryParams,
      );
    }

    return `() => \`\${API_BASE}${path}\``;
  }

  extractPathParams(path) {
    const matches = path.match(/\{([^}]+)\}/g);
    return matches ? matches.map((match) => match.slice(1, -1)) : [];
  }

  replacePathParams(path) {
    return path.replace(/\{([^}]+)\}/g, "${$1}");
  }

  generateQueryParamFunction(path, pathParams, queryParams) {
    const pathParamDefs = pathParams.map((p) => `${p}: string`);
    const queryParamDefs = queryParams.map((p) => {
      const optional = p.required ? "" : "?";
      return `${p.name}${optional}: string`;
    });

    const allParamDefs = [...pathParamDefs, ...queryParamDefs].join("; ");
    const pathWithParamsPrefix = path.replace(/\$\{(\w+)\}/g, "${params.$1}");

    const queryParamLogic =
      queryParams.length > 0
        ? `
      const urlParams = new URLSearchParams();
${queryParams
  .map((p) => {
    const condition = p.required ? `params.${p.name}` : `params.${p.name}`;
    return `      if (${condition}) urlParams.set("${p.name}", params.${p.name});`;
  })
  .join("\n")}
      const query = urlParams.toString();
      return \`\${API_BASE}${pathWithParamsPrefix}\${query ? \`?\${query}\` : ""}\`;`
        : `      return \`\${API_BASE}${pathWithParamsPrefix}\`;`;

    return `(params: { ${allParamDefs} }) => {${queryParamLogic}
    }`;
  }

  writeRoutes(outputPath) {
    const routeEntries = Object.entries(this.routes).map(([group, routes]) => {
      const routeObj = Object.entries(routes)
        .map(([name, func]) => {
          return `    ${name}: ${func}`;
        })
        .join(",\n");

      return `  ${group}: {\n${routeObj},\n  }`;
    });

    const content = `/**
 * This file was auto-generated from OpenAPI spec.
 * Do not make direct changes to the file.
 */

const API_BASE = "${this.apiBase}";

export const routes = {
${routeEntries.join(",\n")}
} as const;

// Type helpers
export type Routes = typeof routes;
export type RouteGroup = keyof Routes;

// Helper to get all route functions from a group
export type RouteFunctions<T extends RouteGroup> = Routes[T];
`;

    writeFileSync(outputPath, content);
  }
}

// CLI usage
if (process.argv[1] === new URL(import.meta.url).pathname) {
  const specPath = process.argv[2] || "openapi.json";
  const outputPath = process.argv[3] || "src/lib/routes.ts";
  const apiBase = process.argv[4] || "/axum";

  const generator = new RouteGenerator(apiBase);
  generator.generate(specPath, outputPath);
}

export default RouteGenerator;
