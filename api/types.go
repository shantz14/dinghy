package api

type Pod struct {
	Kind string
	Metadata ObjectMeta
	Spec PodSpec
}

type ObjectMeta struct {
	Name string
	Labels map[string]string
}

type PodSpec struct {
	Containers []Container
}
 
type Container struct {
	Name string
	Image string
}

