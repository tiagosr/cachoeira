#pragma once

#include <chrono>
#include <utility>
#include <variant>
#include <type_traits>
#include <cassert>

namespace cc {

    struct Time {
        float delta_seconds;
        float delta_real_seconds;
        float fixed_seconds;

        using duration = std::chrono::high_resolution_clock::duration;
        using instant = std::chrono::high_resolution_clock::time_point;
        using duration_in_float_seconds = std::chrono::duration<float, std::ratio<1, 1>>;
        using duration_in_double_seconds = std::chrono::duration<double, std::ratio<1, 1>>;

        duration delta_time;
        duration delta_real_time;
        duration fixed_time;

        float frame_number;
        instant last_fixed_update;
        duration absolute_real_time;
        duration absolute_time;
        float time_scale;


        static constexpr float duration_to_seconds(duration d) {
            return duration_in_float_seconds(d).count();
        }
        static constexpr double duration_to_seconds_double(duration d) {
            return duration_in_double_seconds(d).count();
        }

        static constexpr duration seconds_to_duration(float secs) {
            return std::chrono::duration_cast<duration>(
                duration_in_float_seconds(secs));
        }

        static constexpr duration seconds_to_duration(double secs) {
            return std::chrono::duration_cast<duration>(
                duration_in_double_seconds(secs));
        }

        constexpr double absolute_time_seconds() {
            return duration_to_seconds_double(absolute_time);
        }

        constexpr duration fixed_timestep_default_duration() {
            using namespace std::chrono_literals;
            return duration(16666666ns);
        }

        Time()
            : delta_seconds(0)
            , delta_time(0)
            , delta_real_seconds(0)
            , delta_real_time(0)
            , fixed_seconds(duration_to_seconds(fixed_timestep_default_duration()))
            , fixed_time(fixed_timestep_default_duration())
            , last_fixed_update(std::chrono::high_resolution_clock::now())
            , frame_number(0)
            , absolute_real_time()
            , absolute_time()
            , time_scale(1.0f)
        { }

        void set_delta_seconds(float secs) {
            delta_seconds = secs * time_scale;
            delta_time = seconds_to_duration(secs * time_scale);
            delta_real_seconds = secs;
            delta_real_time = seconds_to_duration(secs);

            absolute_time += delta_time;
            absolute_real_time += delta_real_time;
        }

        void set_delta_time(duration d) {
            delta_seconds = duration_to_seconds(d) * time_scale;
            delta_time = seconds_to_duration(duration_to_seconds(d) * time_scale);
            delta_real_seconds = duration_to_seconds(d);
            delta_time = d;

            absolute_time += delta_time;
            absolute_real_time += delta_real_time;
        }

        void set_fixed_seconds(float seconds) {
            fixed_seconds = seconds;
            fixed_time = seconds_to_duration(seconds);
        }

        void set_fixed_time(duration d) {
            fixed_seconds = duration_to_seconds(d);
            fixed_time = d;
        }

        void increment_frame_number() {
            frame_number++;
        }

        void set_time_scale(float multiplier) {
            assert(multiplier != 0);
            assert(multiplier != std::numeric_limits<float>::infinity());

            time_scale = multiplier;
        }

        void finish_fixed_update() {
            last_fixed_update += fixed_time;
        }
    };

    struct Stopwatch {
        struct Waiting {};
        struct Started {
            Time::duration duration;
            Time::instant instant;
        };
        struct Ended {
            Time::duration duration;
        };

        typedef
            std::variant<
            Waiting,
            Started,
            Ended
            > stopwatch;
        stopwatch state;
        
        Stopwatch() { reset(); }

        Time::duration elapsed() {
            struct TimeElapsedVisitor {
                Time::duration operator()(const Waiting&) const { return Time::duration(0); }
                Time::duration operator()(const Started& state) const { return state.instant.time_since_epoch() + state.duration; }
                Time::duration operator()(const Ended& state) const { return state.duration; }
            };
            return std::visit(TimeElapsedVisitor(), state);
        }

        void restart() {
            state = Started{ Time::duration(), Time::instant::time_point() };
        }

        void start() {
            if (std::holds_alternative<Waiting>(state)) {
                restart();
            }
            else if (std::holds_alternative<Ended>(state)) {
                Ended ended = std::get<Ended>(state);
                state = Started{ ended.duration, Time::instant::time_point() };
            }
        }

        void reset() {
            state = Waiting();
        }

    };


}
