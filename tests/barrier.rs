extern crate pulse;

use std::thread;
use pulse::*;

#[test]
fn using_vec() {
    let mut pulses = Vec::new();
    let mut triggers = Vec::new();
    for _ in 0..8 {
        let (p, t) = Pulse::new();
        pulses.push(p);
        triggers.push(t);
    }

    let barrier = Barrier::new(pulses);
    let pulse = barrier.pulse();

    let last_trigger = triggers.pop().unwrap();
    for t in triggers {
        t.pulse();
        assert!(pulse.is_pending());
    }

    last_trigger.pulse();
    assert!(!pulse.is_pending());
}

#[test]
fn using_slice() {
    let mut pulses = Vec::new();
    let mut triggers = Vec::new();
    for _ in 0..8 {
        let (p, t) = Pulse::new();
        pulses.push(p);
        triggers.push(t);
    }

    let barrier = Barrier::new(&pulses[..]);
    let pulse = barrier.pulse();

    let last_trigger = triggers.pop().unwrap();
    for t in triggers {
        t.pulse();
        assert!(pulse.is_pending());
    }

    last_trigger.pulse();
    assert!(!pulse.is_pending());
}

#[test]
fn empty() {
    let barrier = Barrier::new([]);
    let pulse = barrier.pulse();
    assert!(!pulse.is_pending());
}

#[test]
fn using_threads() {
    let mut pulses = Vec::new();
    let mut triggers = Vec::new();
    for _ in 0..8 {
        let (p, t) = Pulse::new();
        pulses.push(p);
        triggers.push(t);
    }

    let barrier = Barrier::new(pulses);
    let pulse = barrier.pulse();

    thread::spawn(move || {
        for t in triggers {
            t.pulse();
        }
    });

    pulse.wait().unwrap();
}

#[test]
fn dropped_barrier() {
    let mut pulses = Vec::new();
    let mut triggers = Vec::new();
    for _ in 0..8 {
        let (p, t) = Pulse::new();
        pulses.push(p);
        triggers.push(t);
    }

    let pulse = {
        let barrier = Barrier::new(&pulses[..]);
        barrier.pulse()
    };

    let last_trigger = triggers.pop().unwrap();
    for t in triggers {
        t.pulse();
        assert!(pulse.is_pending());
    }

    last_trigger.pulse();
    assert!(!pulse.is_pending());   
}