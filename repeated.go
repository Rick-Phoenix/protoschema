package protoschema

import (
	"errors"
	"fmt"
	"maps"
)

// An instance of a repeated protobuf field.
type RepeatedField struct {
	name     string
	field    FieldBuilder
	unique   bool
	minItems *uint
	maxItems *uint
	*ProtoField[RepeatedField]
}

// The constructor for a protobuf repeated field. The name indicated in the FieldBuilder constructor itself will be ignored, while the first argument will be used.
func Repeated(name string, b FieldBuilder) *RepeatedField {
	options := make(map[string]any)
	rules := make(map[string]any)
	self := &RepeatedField{
		field: b, name: name,
	}

	self.ProtoField = &ProtoField[RepeatedField]{protoFieldInternal: &protoFieldInternal{
		options: options, rules: rules, repeated: true, goType: "[]" + b.GetGoType(), name: name, isNonScalar: true, messageRef: b.GetMessageRef(),
	}, self: self}

	return self
}

// The method that processes the field's schema and returns its data. Used to satisfy the FieldBuilder interface. Mostly for internal use.
func (b *RepeatedField) Build(fieldNr uint32, imports Set) (FieldData, error) {
	fieldData, err := b.field.Build(fieldNr, imports)

	err = errors.Join(err, b.errors)

	if fieldData.Optional {
		fmt.Printf("Ignoring 'optional' for repeated field %q...", b.name)
	}

	if b.unique {
		if fieldData.IsNonScalar {
			err = errors.Join(err, fmt.Errorf("Cannot apply contraint 'unique' to a non-scalar repeated field."))
		}
	}

	if fieldData.IsMap {
		err = errors.Join(err, fmt.Errorf("Map fields cannot be repeated (must be wrapped in a message type)"))
	}

	if fieldData.Repeated {
		err = errors.Join(err, fmt.Errorf("Cannot nest repeated fields inside one another (must be wrapped inside a message type first)"))
	}

	if fieldData.Required {
		fmt.Printf("Ignoring ineffective 'required' option for repeated field '%s' (you can set min_len to 1 instead to require at least one element)", b.name)
	}

	options := make([]string, len(b.repeatedOptions))
	copy(options, b.repeatedOptions)

	if len(fieldData.Rules) > 0 {
		rulesMap := make(map[string]any)
		rulesCopy := make(map[string]any)
		maps.Copy(rulesCopy, fieldData.Rules)
		rulesMap[fieldData.ProtoBaseType] = rulesCopy

		stringRules, fmtErr := formatProtoValue(rulesMap)
		if fmtErr != nil {
			err = errors.Join(err, fmtErr)
		}

		options = append(options, fmt.Sprintf("(buf.validate.field).repeated.items = %s", stringRules))
	}

	options, optErr := getOptions(b.options, options)

	if optErr != nil {
		err = errors.Join(err, optErr)
	}

	if err != nil {
		return FieldData{}, err
	}

	return FieldData{Name: b.name, ProtoType: fieldData.ProtoType, GoType: b.goType, Optional: fieldData.Optional, FieldNr: fieldNr, Repeated: true, Options: options, IsNonScalar: true, MessageRef: fieldData.MessageRef}, nil
}

// Rule: this repeated field must contain unique values. Causes an error if the fields are non-scalar.
func (b *RepeatedField) Unique() *RepeatedField {
	b.options["(buf.validate.field).repeated.unique"] = true
	b.unique = true
	return b
}

// Rule: this repeated field must have at least the specified number of items.
func (b *RepeatedField) MinItems(n uint) *RepeatedField {
	if b.maxItems != nil && *b.maxItems < n {
		b.errors = errors.Join(b.errors, fmt.Errorf("max_items cannot be smaller than min_items."))
	}

	b.options["(buf.validate.field).repeated.min_items"] = n

	b.minItems = &n
	return b
}

// Rule: this repeated field must have no more than the specified number of items.
func (b *RepeatedField) MaxItems(n uint) *RepeatedField {
	if b.minItems != nil && *b.minItems > n {
		b.errors = errors.Join(b.errors, fmt.Errorf("max_items cannot be smaller than min_items."))
	}

	b.options["(buf.validate.field).repeated.max_items"] = n

	b.maxItems = &n
	return b
}
