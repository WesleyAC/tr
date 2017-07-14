use object::Object;

struct Scene<'a> {
    objects: Vec<&'a Object>
}
