#[cfg(test)]
mod tests {
    use dockspy::container::{analyze_logs, parse_container_info};

    #[test]
    fn test_analyze_logs() {
        let log_data = "2019-01-01 00:00:00,000 ERROR [main] org.apache.catalina.core.ContainerBase.[Tomcat].[localhost].[/] - Exception sending context initialized event to listener instance of class [org.springframework.web.context.ContextLoaderListener]
org.springframework.beans.factory.BeanCreationException: Error creating bean with name 'org.springframework.web.servlet.mvc.method.annotation.RequestMappingHandlerMapping#0' defined in class path resource [org/springframework/boot/autoconfigure/web/servlet/WebMvcAutoConfiguration$EnableWebMvcConfiguration.class]: Invocation of init method failed; nested exception is java.lang.IllegalStateException: Ambiguous mapping. Cannot map 'consumerController' method
";
        let errors = analyze_logs(log_data);
        assert_eq!(errors.len(), 2);
    }

    #[test]
    fn test_parse_container_info() {
        let example_output_ps = "3d3881a67b11    docker-api-1
b4bfeded8034    docker-consumer_indexer-1
6a1971325713    docker-storage-1
3ced363bd4d0    docker-elastic-1
e6f71bb00d43    docker-broker-1
";
        let containers = parse_container_info(example_output_ps);
        assert_eq!(containers.len(), 5);
        assert_eq!(containers[0].id, "3d3881a67b11");
        assert_eq!(containers[0].name, "docker-api-1");
    }
}
