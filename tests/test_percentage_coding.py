"""Python tests for the percentage coder bindings on ``feagi_rust_py_libs.connector_core``.

Run after ``maturin develop`` with ``pytest tests/test_percentage_coding.py``. The units are passed
exactly as the agent registration JSON carries them; Python never selects a coding variant.
"""

import json

import pytest

try:
    from feagi_rust_py_libs import connector_core
except ImportError as exc:  # no fallback: a missing build is a failure, not a skip
    raise ImportError(
        "feagi_rust_py_libs is not built; run `maturin develop --release` in feagi-rust-py-libs"
    ) from exc

OUT = "output_units_and_decoder_properties"
IN = "input_units_and_encoder_properties"
FLAGS = {"frame_change_handling": "Absolute", "percentage_neuron_positioning": "Linear"}


def unit(flags: dict = FLAGS, index: int = 0) -> str:
    """Unit definition JSON as the registration carries it."""
    return json.dumps({"cortical_unit_index": index, "io_configuration_flags": flags})


def test_bindings_are_exposed() -> None:
    for name in (
        "percentage_coding_variant",
        "decode_percentage_from_z_indexes",
        "encode_percentage_to_z_indexes",
        "target_to_z_bin",
        "validate_physical_range",
    ):
        assert callable(getattr(connector_core, name))


def test_bin_170_at_depth_511_decodes_like_the_rust_decoder_and_target_inverts_it() -> None:
    value = connector_core.decode_percentage_from_z_indexes(OUT, "PositionalServo", unit(), 511, [170])
    assert value == pytest.approx(1.0 - 170.0 / 510.0, abs=1e-6)
    assert connector_core.target_to_z_bin(OUT, "PositionalServo", unit(), 511, value) == 170


def test_target_to_z_bin_inverts_the_decoder_for_every_bin() -> None:
    depth = 64
    for z in range(depth):
        value = connector_core.decode_percentage_from_z_indexes(OUT, "PositionalServo", unit(), depth, [z])
        assert connector_core.target_to_z_bin(OUT, "PositionalServo", unit(), depth, value) == z


def test_variant_comes_from_the_unit_and_its_flags() -> None:
    assert connector_core.percentage_coding_variant(OUT, "PositionalServo", unit()) == "UnsignedLinear"
    fractional = dict(FLAGS, percentage_neuron_positioning="Fractional")
    assert connector_core.percentage_coding_variant(OUT, "PositionalServo", unit(fractional)) == "UnsignedFractional"
    assert connector_core.percentage_coding_variant(OUT, "RotaryMotor", unit()) == "SignedLinearAlongZ"


def test_area_index_selects_the_unit_area() -> None:
    incremental = dict(FLAGS, frame_change_handling="Incremental")
    assert connector_core.percentage_coding_variant(OUT, "PositionalServo", unit(incremental), 1) == "UnsignedLinear"


def test_signed_along_z_round_trip_through_target_bin() -> None:
    bin_ = connector_core.target_to_z_bin(OUT, "RotaryMotor", unit(), 9, 0.0)
    assert bin_ == 4
    assert connector_core.decode_percentage_from_z_indexes(OUT, "RotaryMotor", unit(), 9, [bin_]) == 0.0


def test_encode_returns_positive_and_negative_lists() -> None:
    positive, negative = connector_core.encode_percentage_to_z_indexes(OUT, "RotaryMotor", unit(), 9, 0.0)
    assert positive == [4]
    assert negative == []


def test_negative_lobe_is_rejected_for_single_column_variants() -> None:
    with pytest.raises(ValueError):
        connector_core.decode_percentage_from_z_indexes(OUT, "PositionalServo", unit(), 8, [0], [1])


@pytest.mark.parametrize(
    "call",
    [
        lambda: connector_core.decode_percentage_from_z_indexes(OUT, "NoSuchUnit", unit(), 8, [0]),
        lambda: connector_core.decode_percentage_from_z_indexes("bogus", "PositionalServo", unit(), 8, [0]),
        lambda: connector_core.decode_percentage_from_z_indexes(OUT, "PositionalServo", "{not json", 8, [0]),
        lambda: connector_core.decode_percentage_from_z_indexes(OUT, "PositionalServo", unit({}), 8, [0]),
        lambda: connector_core.decode_percentage_from_z_indexes(OUT, "PositionalServo", unit(), 0, [0]),
        lambda: connector_core.decode_percentage_from_z_indexes(OUT, "PositionalServo", unit(), 8, [8]),
        lambda: connector_core.decode_percentage_from_z_indexes(OUT, "PositionalServo", unit(), 8, []),
        lambda: connector_core.decode_percentage_from_z_indexes(OUT, "PositionalServo", unit(), 8, [0], None, 9),
        lambda: connector_core.encode_percentage_to_z_indexes(OUT, "PositionalServo", unit(), 8, 1.5),
        lambda: connector_core.target_to_z_bin(OUT, "PositionalServo", unit(), 8, -0.1),
        lambda: connector_core.target_to_z_bin(OUT, "PositionalServo", unit(), 8, float("nan")),
        lambda: connector_core.target_to_z_bin(IN, "NoSuchSensor", unit(), 8, 0.5),
    ],
)
def test_bad_inputs_raise_value_error_with_a_message(call) -> None:
    with pytest.raises(ValueError) as caught:
        call()
    assert str(caught.value)


def test_physical_range_is_validated_by_the_core_rules() -> None:
    ok = json.dumps({"value_at_0": -1.0, "value_at_1": 1.0, "unit": "rad"})
    assert connector_core.validate_physical_range(ok) == (-1.0, 1.0, "rad")
    for bad in (
        {"value_at_0": 1.0, "value_at_1": 1.0, "unit": "rad"},
        {"value_at_0": 0.0, "value_at_1": 1.0, "unit": " "},
        {"value_at_0": 0.0, "value_at_1": 1.0},
    ):
        with pytest.raises(ValueError):
            connector_core.validate_physical_range(json.dumps(bad))
